//! Authentication providers for OpenAPI tool execution.

use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Trait for applying authentication to HTTP requests.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()>;
    fn name(&self) -> &str;
}

/// No authentication required.
#[derive(Debug, Clone)]
pub struct NoAuth;

#[async_trait]
impl AuthProvider for NoAuth {
    async fn apply_auth(&self, _req: &mut reqwest::Request) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "none"
    }
}

/// API key authentication (header, query, or cookie).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAuth {
    pub key: String,
    pub location: KeyLocation,
    pub param_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KeyLocation {
    Header,
    Query,
    Cookie,
}

impl ApiKeyAuth {
    pub fn new(key: String, location: KeyLocation, param_name: String) -> Self {
        Self {
            key,
            location,
            param_name,
        }
    }

    pub fn header(key: String, header_name: String) -> Self {
        Self::new(key, KeyLocation::Header, header_name)
    }

    pub fn query(key: String, param_name: String) -> Self {
        Self::new(key, KeyLocation::Query, param_name)
    }
}

#[async_trait]
impl AuthProvider for ApiKeyAuth {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()> {
        match self.location {
            KeyLocation::Header => {
                req.headers_mut().insert(
                    reqwest::header::HeaderName::from_bytes(self.param_name.as_bytes())
                        .context("invalid header name")?,
                    reqwest::header::HeaderValue::from_str(&self.key)
                        .context("invalid header value")?,
                );
            }
            KeyLocation::Query => {
                let url = req.url_mut();
                url.query_pairs_mut()
                    .append_pair(&self.param_name, &self.key);
            }
            KeyLocation::Cookie => {
                let cookie_value = format!("{}={}", self.param_name, self.key);
                req.headers_mut().insert(
                    reqwest::header::COOKIE,
                    reqwest::header::HeaderValue::from_str(&cookie_value)
                        .context("invalid cookie value")?,
                );
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "api_key"
    }
}

/// Bearer token authentication (Authorization: Bearer <token>).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearerAuth {
    pub token: String,
}

impl BearerAuth {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[async_trait]
impl AuthProvider for BearerAuth {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()> {
        let value = format!("Bearer {}", self.token);
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&value).context("invalid bearer token")?,
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "bearer"
    }
}

/// HTTP Basic authentication (Authorization: Basic <base64>).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[async_trait]
impl AuthProvider for BasicAuth {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()> {
        let credentials = format!("{}:{}", self.username, self.password);
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            credentials.as_bytes(),
        );
        let value = format!("Basic {}", encoded);
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&value)
                .context("invalid basic auth credentials")?,
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "basic"
    }
}

/// OAuth2 token response from token endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// OAuth2 authentication with automatic token refresh.
#[derive(Clone)]
pub struct OAuth2Auth {
    client_id: String,
    client_secret: String,
    token_url: String,
    scopes: Vec<String>,
    token_cache: Arc<Mutex<Option<(TokenResponse, Instant)>>>,
}

impl OAuth2Auth {
    pub fn new(
        client_id: String,
        client_secret: String,
        token_url: String,
        scopes: Vec<String>,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            token_url,
            scopes,
            token_cache: Arc::new(Mutex::new(None)),
        }
    }

    /// Get a valid access token, refreshing if necessary.
    pub async fn get_token(&self) -> Result<String> {
        // Check if we have a valid cached token
        {
            let cache = self.token_cache.lock();
            if let Some((token, fetched_at)) = cache.as_ref() {
                if let Some(expires_in) = token.expires_in {
                    let expiry = fetched_at.checked_add(Duration::from_secs(expires_in));
                    let now = Instant::now();
                    // Refresh 60 seconds before expiry
                    if let Some(expiry) = expiry {
                        if now
                            < expiry
                                .checked_sub(Duration::from_secs(60))
                                .unwrap_or(expiry)
                        {
                            return Ok(token.access_token.clone());
                        }
                    }
                }
            }
        } // Lock released here

        // Token expired or not cached, fetch new one
        let token = self.fetch_token().await?;
        let fetched_at = Instant::now();

        let access_token = token.access_token.clone();
        *self.token_cache.lock() = Some((token, fetched_at));

        Ok(access_token)
    }

    async fn fetch_token(&self) -> Result<TokenResponse> {
        let client = reqwest::Client::new();
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("grant_type", "client_credentials".to_string());
        params.insert("client_id", self.client_id.clone());
        params.insert("client_secret", self.client_secret.clone());

        if !self.scopes.is_empty() {
            let scope_str = self.scopes.join(" ");
            params.insert("scope", scope_str);
        }

        let response = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .context("failed to request OAuth2 token")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!(
                "OAuth2 token request failed with status {}: {}",
                status,
                body
            );
        }

        let token: TokenResponse = response
            .json()
            .await
            .context("failed to parse OAuth2 token response")?;

        Ok(token)
    }

    pub async fn refresh_token(&self) -> Result<String> {
        // Clear cache to force refresh
        *self.token_cache.lock() = None;
        self.get_token().await
    }
}

#[async_trait]
impl AuthProvider for OAuth2Auth {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()> {
        let token = self.get_token().await?;
        let value = format!("Bearer {}", token);
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&value).context("invalid OAuth2 token")?,
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "oauth2"
    }
}

/// Create an auth provider from configuration.
pub fn create_auth_provider(
    auth_type: &str,
    config: &HashMap<String, String>,
) -> Result<Arc<dyn AuthProvider>> {
    match auth_type {
        "none" => Ok(Arc::new(NoAuth)),
        "api_key" => {
            let key = config
                .get("key")
                .ok_or_else(|| anyhow::anyhow!("api_key auth requires 'key' config"))?
                .clone();
            let location = config
                .get("location")
                .map(|loc| match loc.as_str() {
                    "header" => KeyLocation::Header,
                    "query" => KeyLocation::Query,
                    "cookie" => KeyLocation::Cookie,
                    _ => KeyLocation::Header,
                })
                .unwrap_or(KeyLocation::Header);
            let param_name = config
                .get("param_name")
                .cloned()
                .unwrap_or_else(|| "X-API-Key".to_string());

            Ok(Arc::new(ApiKeyAuth::new(key, location, param_name)))
        }
        "bearer" => {
            let token = config
                .get("token")
                .ok_or_else(|| anyhow::anyhow!("bearer auth requires 'token' config"))?
                .clone();
            Ok(Arc::new(BearerAuth::new(token)))
        }
        "basic" => {
            let username = config
                .get("username")
                .ok_or_else(|| anyhow::anyhow!("basic auth requires 'username' config"))?
                .clone();
            let password = config
                .get("password")
                .ok_or_else(|| anyhow::anyhow!("basic auth requires 'password' config"))?
                .clone();
            Ok(Arc::new(BasicAuth::new(username, password)))
        }
        "oauth2" => {
            let client_id = config
                .get("client_id")
                .ok_or_else(|| anyhow::anyhow!("oauth2 auth requires 'client_id' config"))?
                .clone();
            let client_secret = config
                .get("client_secret")
                .ok_or_else(|| anyhow::anyhow!("oauth2 auth requires 'client_secret' config"))?
                .clone();
            let token_url = config
                .get("token_url")
                .ok_or_else(|| anyhow::anyhow!("oauth2 auth requires 'token_url' config"))?
                .clone();
            let scopes = config
                .get("scopes")
                .map(|s| s.split(',').map(|scope| scope.trim().to_string()).collect())
                .unwrap_or_default();

            Ok(Arc::new(OAuth2Auth::new(
                client_id,
                client_secret,
                token_url,
                scopes,
            )))
        }
        _ => bail!("unsupported auth type: {}", auth_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn no_auth_does_nothing() {
        let auth = NoAuth;
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, "https://example.com".parse().unwrap());
        auth.apply_auth(&mut req).await.unwrap();
        assert!(req.headers().get(reqwest::header::AUTHORIZATION).is_none());
    }

    #[tokio::test]
    async fn api_key_header() {
        let auth = ApiKeyAuth::header("test-key".to_string(), "X-API-Key".to_string());
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, "https://example.com".parse().unwrap());
        auth.apply_auth(&mut req).await.unwrap();
        assert_eq!(req.headers().get("X-API-Key").unwrap(), "test-key");
    }

    #[tokio::test]
    async fn api_key_query() {
        let auth = ApiKeyAuth::query("test-key".to_string(), "api_key".to_string());
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, "https://example.com".parse().unwrap());
        auth.apply_auth(&mut req).await.unwrap();
        assert!(req.url().query().unwrap().contains("api_key=test-key"));
    }

    #[tokio::test]
    async fn bearer_auth() {
        let auth = BearerAuth::new("test-token".to_string());
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, "https://example.com".parse().unwrap());
        auth.apply_auth(&mut req).await.unwrap();
        assert_eq!(
            req.headers().get(reqwest::header::AUTHORIZATION).unwrap(),
            "Bearer test-token"
        );
    }

    #[tokio::test]
    async fn basic_auth() {
        let auth = BasicAuth::new("user".to_string(), "pass".to_string());
        let mut req =
            reqwest::Request::new(reqwest::Method::GET, "https://example.com".parse().unwrap());
        auth.apply_auth(&mut req).await.unwrap();
        let header = req.headers().get(reqwest::header::AUTHORIZATION).unwrap();
        assert!(header.to_str().unwrap().starts_with("Basic "));
    }
}
