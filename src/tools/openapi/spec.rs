use super::validator::{OpenApiValidator, ValidationReport};
use anyhow::{Context, Result};
use openapiv3::OpenAPI;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Authentication modes detected from OpenAPI security definitions.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    None,
    ApiKey,
    Bearer,
    Basic,
    OAuth2,
    OpenIdConnect,
    Unknown,
}

/// Default authentication configuration associated with a harvested spec.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthConfig {
    pub auth_type: AuthType,
    pub token_env: Option<String>,
    pub param_name: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            auth_type: AuthType::None,
            token_env: None,
            param_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    /// Parsed OpenAPI document
    pub spec: OpenAPI,
    /// Base URL for API requests
    pub base_url: String,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Metadata about this spec
    pub metadata: SpecMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecMetadata {
    pub provider: String,
    pub service: String,
    pub version: String,
    pub tier: SpecTier,
    pub quality_score: u8,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum SpecTier {
    Native,
    Verified,
    Community,
    Experimental,
}

impl OpenApiSpec {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read OpenAPI file {}", path.display()))?;

        // Try to parse as OpenAPI 3.0 first
        let spec_result: Result<OpenAPI, _> = if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            serde_json::from_str(&contents)
        } else {
            serde_yaml::from_str(&contents).or_else(|_| serde_json::from_str(&contents))
        };

        let spec = match spec_result {
            Ok(spec) => spec,
            Err(_) => {
                // Check if it's Swagger 2.0 by looking for "swagger": "2.0"
                if contents.contains("\"swagger\"")
                    && (contents.contains("\"2.0\"") || contents.contains("'2.0'"))
                {
                    // Try to convert using npx swagger2openapi
                    let output = std::process::Command::new("npx")
                        .args(&[
                            "--yes",
                            "swagger2openapi",
                            path.to_str().unwrap(),
                            "-o",
                            "-",
                        ])
                        .output();

                    match output {
                        Ok(result) if result.status.success() => {
                            let converted = String::from_utf8_lossy(&result.stdout);
                            serde_json::from_str(&converted)
                                .or_else(|_| serde_yaml::from_str(&converted))
                                .with_context(|| {
                                    format!("failed to parse converted OpenAPI {}", path.display())
                                })?
                        }
                        _ => {
                            // Fallback: skip Swagger 2.0 specs for now
                            anyhow::bail!(
                                "Swagger 2.0 spec detected but conversion failed (npx swagger2openapi not available)"
                            );
                        }
                    }
                } else {
                    anyhow::bail!("failed to parse as OpenAPI 3.0");
                }
            }
        };

        let mut parsed = Self::from_openapi(spec);
        parsed.metadata.source = format!("file://{}", path.display());
        Ok(parsed)
    }

    pub fn from_url(url: &str) -> Result<Self> {
        let body = reqwest::blocking::get(url)
            .with_context(|| format!("failed to fetch OpenAPI URL {url}"))?
            .text()
            .context("failed to read OpenAPI response body")?;
        let spec: OpenAPI = serde_yaml::from_str(&body)
            .or_else(|_| serde_json::from_str(&body))
            .with_context(|| format!("failed to parse OpenAPI response from {url}"))?;
        let mut parsed = Self::from_openapi(spec);
        parsed.metadata.source = url.to_string();
        Ok(parsed)
    }

    pub fn validate(&self) -> Result<ValidationReport> {
        OpenApiValidator::validate(&self.spec)
    }

    pub fn dedup_key(&self) -> String {
        format!(
            "{}::{}::{}",
            self.metadata.provider, self.metadata.service, self.metadata.version
        )
    }

    pub fn from_openapi(spec: OpenAPI) -> Self {
        let base_url = spec
            .servers
            .first()
            .map(|server| server.url.clone())
            .unwrap_or_default();
        let title = spec.info.title.trim();
        let service = if title.is_empty() {
            "unknown-service".to_string()
        } else {
            title.to_string()
        };
        let version = if spec.info.version.trim().is_empty() {
            "unknown-version".to_string()
        } else {
            spec.info.version.clone()
        };

        let auth = OpenApiValidator::detect_auth_type(&spec)
            .into_iter()
            .next()
            .map(|auth_type| AuthConfig {
                auth_type,
                ..AuthConfig::default()
            })
            .unwrap_or_default();

        Self {
            spec,
            base_url,
            auth,
            metadata: SpecMetadata {
                provider: "unknown-provider".to_string(),
                service,
                version,
                tier: SpecTier::Community,
                quality_score: 0,
                source: String::new(),
            },
        }
    }
}
