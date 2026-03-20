//! Runtime executor for OpenAPI operations as tools.

use super::auth::AuthProvider;
use super::spec::OpenApiSpec;
use crate::tools::traits::{Tool, ToolResult};
use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use openapiv3::{Operation, Parameter, ParameterSchemaOrContent, ReferenceOr};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

/// An executable OpenAPI operation wrapped as a Tool.
pub struct OpenApiTool {
    pub(crate) spec: Arc<OpenApiSpec>,
    operation_id: String,
    path: String,
    method: String,
    operation: Operation,
    auth_provider: Option<Arc<dyn AuthProvider>>,
}

impl OpenApiTool {
    pub fn new(
        spec: Arc<OpenApiSpec>,
        operation_id: String,
        path: String,
        method: String,
        operation: Operation,
        auth_provider: Option<Arc<dyn AuthProvider>>,
    ) -> Self {
        Self {
            spec,
            operation_id,
            path,
            method,
            operation,
            auth_provider,
        }
    }

    /// Build an HTTP request from tool arguments.
    fn build_request(&self, args: &Value) -> Result<reqwest::Request> {
        let mut url = self.spec.base_url.clone();
        if !url.ends_with('/') && !self.path.starts_with('/') {
            url.push('/');
        }
        url.push_str(&self.path);

        let mut path_params = HashMap::new();
        let mut query_params = Vec::new();
        let mut headers = HashMap::new();
        let mut body: Option<Value> = None;

        // Extract parameters from args
        if let Some(args_obj) = args.as_object() {
            for (key, value) in args_obj {
                // Try to find matching parameter
                if let Some(param) = self.find_parameter(key) {
                    match param {
                        Parameter::Query { parameter_data, .. } => {
                            let param_name = &parameter_data.name;
                            query_params.push((param_name.clone(), value_to_string(value)));
                        }
                        Parameter::Header { parameter_data, .. } => {
                            let header_name = &parameter_data.name;
                            headers.insert(header_name.clone(), value_to_string(value));
                        }
                        Parameter::Path { parameter_data, .. } => {
                            let param_name = &parameter_data.name;
                            path_params.insert(param_name.clone(), value_to_string(value));
                        }
                        Parameter::Cookie { .. } => {
                            // Cookie parameters not yet supported
                        }
                    }
                } else if key == "body" || key == "requestBody" {
                    body = Some(value.clone());
                }
            }
        }

        // Replace path parameters
        for (param_name, param_value) in path_params {
            let placeholder = format!("{{{}}}", param_name);
            url = url.replace(&placeholder, &param_value);
        }

        // Build URL with query params
        let mut url = reqwest::Url::parse(&url).context("invalid base URL")?;
        for (key, value) in query_params {
            url.query_pairs_mut().append_pair(&key, &value);
        }

        // Build request
        let method = reqwest::Method::from_bytes(self.method.to_uppercase().as_bytes())
            .context("invalid HTTP method")?;

        let mut req = reqwest::Request::new(method, url);

        // Add headers
        for (key, value) in headers {
            req.headers_mut().insert(
                reqwest::header::HeaderName::from_bytes(key.as_bytes())
                    .context("invalid header name")?,
                reqwest::header::HeaderValue::from_str(&value).context("invalid header value")?,
            );
        }

        // Add body if present
        if let Some(body_value) = body {
            let body_str = serde_json::to_string(&body_value)?;
            *req.body_mut() = Some(body_str.into());
            req.headers_mut().insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
        }

        Ok(req)
    }

    fn find_parameter(&self, name: &str) -> Option<Parameter> {
        for param_ref in &self.operation.parameters {
            if let ReferenceOr::Item(param) = param_ref {
                let param_name = match param {
                    Parameter::Query { parameter_data, .. } => &parameter_data.name,
                    Parameter::Header { parameter_data, .. } => &parameter_data.name,
                    Parameter::Path { parameter_data, .. } => &parameter_data.name,
                    Parameter::Cookie { parameter_data, .. } => &parameter_data.name,
                };
                if param_name == name {
                    return Some(param.clone());
                }
            }
        }
        None
    }

    /// Convert OpenAPI operation to JSON Schema for tool parameters.
    fn operation_to_json_schema(&self) -> Value {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        // Add parameters
        for param_ref in &self.operation.parameters {
            if let ReferenceOr::Item(param) = param_ref {
                let (param_name, param_required, param_schema) = match param {
                    Parameter::Query {
                        parameter_data,
                        allow_reserved: _,
                        style: _,
                        allow_empty_value: _,
                    } => (
                        &parameter_data.name,
                        parameter_data.required,
                        parameter_data_to_schema(parameter_data),
                    ),
                    Parameter::Header {
                        parameter_data,
                        style: _,
                    } => (
                        &parameter_data.name,
                        parameter_data.required,
                        parameter_data_to_schema(parameter_data),
                    ),
                    Parameter::Path {
                        parameter_data,
                        style: _,
                    } => (
                        &parameter_data.name,
                        parameter_data.required,
                        parameter_data_to_schema(parameter_data),
                    ),
                    Parameter::Cookie {
                        parameter_data,
                        style: _,
                    } => (
                        &parameter_data.name,
                        parameter_data.required,
                        parameter_data_to_schema(parameter_data),
                    ),
                };

                properties.insert(param_name.clone(), param_schema);
                if param_required {
                    required.push(param_name.clone());
                }
            }
        }

        // Add request body if present
        if let Some(ReferenceOr::Item(request_body)) = &self.operation.request_body {
            properties.insert(
                "body".to_string(),
                json!({
                    "type": "object",
                    "description": request_body.description.as_deref().unwrap_or("Request body")
                }),
            );
            if request_body.required {
                required.push("body".to_string());
            }
        }

        json!({
            "type": "object",
            "properties": properties,
            "required": required
        })
    }

    async fn execute_http(&self, mut req: reqwest::Request) -> Result<Value> {
        // Apply authentication
        if let Some(auth) = &self.auth_provider {
            auth.apply_auth(&mut req).await?;
        }

        // Execute request
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        let response = client.execute(req).await.context("HTTP request failed")?;

        let status = response.status();
        let body = response.text().await.context("failed to read response body")?;

        // Try to parse as JSON, fall back to text
        let result = if let Ok(json_value) = serde_json::from_str::<Value>(&body) {
            json!({
                "status": status.as_u16(),
                "body": json_value
            })
        } else {
            json!({
                "status": status.as_u16(),
                "body": body
            })
        };

        if !status.is_success() {
            bail!("HTTP request failed with status {}: {}", status, body);
        }

        Ok(result)
    }
}

#[async_trait]
impl Tool for OpenApiTool {
    fn name(&self) -> &str {
        &self.operation_id
    }

    fn description(&self) -> &str {
        self.operation
            .summary
            .as_deref()
            .or(self.operation.description.as_deref())
            .unwrap_or("OpenAPI operation")
    }

    fn parameters_schema(&self) -> Value {
        self.operation_to_json_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let req = self.build_request(&args)?;
        
        match self.execute_http(req).await {
            Ok(result) => Ok(ToolResult {
                success: true,
                output: serde_json::to_string_pretty(&result)?,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("{:#}", e)),
            }),
        }
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => value.to_string(),
    }
}

fn parameter_data_to_schema(
    parameter_data: &openapiv3::ParameterData,
) -> Value {
    let mut schema = json!({
        "description": parameter_data.description.as_deref().unwrap_or("")
    });

    // Extract schema from format
    if let ParameterSchemaOrContent::Schema(schema_ref) = &parameter_data.format {
        if let ReferenceOr::Item(openapi_schema) = schema_ref {
            // Convert OpenAPI schema to JSON Schema (simplified)
            match &openapi_schema.schema_kind {
                openapiv3::SchemaKind::Type(type_) => match type_ {
                    openapiv3::Type::String(_) => {
                        schema["type"] = json!("string");
                    }
                    openapiv3::Type::Number(_) => {
                        schema["type"] = json!("number");
                    }
                    openapiv3::Type::Integer(_) => {
                        schema["type"] = json!("integer");
                    }
                    openapiv3::Type::Boolean(_) => {
                        schema["type"] = json!("boolean");
                    }
                    openapiv3::Type::Array(_) => {
                        schema["type"] = json!("array");
                    }
                    openapiv3::Type::Object(_) => {
                        schema["type"] = json!("object");
                    }
                },
                _ => {
                    schema["type"] = json!("string");
                }
            }
        }
    }

    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_to_string_converts_types() {
        assert_eq!(value_to_string(&json!("hello")), "hello");
        assert_eq!(value_to_string(&json!(42)), "42");
        assert_eq!(value_to_string(&json!(true)), "true");
        assert_eq!(value_to_string(&json!(false)), "false");
    }
}
