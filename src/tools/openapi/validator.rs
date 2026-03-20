use super::spec::AuthType;
use anyhow::Result;
use openapiv3::{OpenAPI, SecurityScheme};
use serde::{Deserialize, Serialize};

pub struct OpenApiValidator;

impl OpenApiValidator {
    pub fn validate(spec: &OpenAPI) -> Result<ValidationReport> {
        let missing_fields = Self::check_required_fields(spec);
        let auth_types = Self::detect_auth_type(spec);
        let quality_score = Self::score_quality(spec);
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        if spec.paths.paths.is_empty() {
            errors.push("spec has no paths".to_string());
        }

        let missing_operation_ids = spec
            .paths
            .paths
            .values()
            .filter_map(|item| item.as_item())
            .flat_map(|item| {
                [
                    item.get.as_ref(),
                    item.put.as_ref(),
                    item.post.as_ref(),
                    item.delete.as_ref(),
                    item.patch.as_ref(),
                    item.options.as_ref(),
                    item.head.as_ref(),
                    item.trace.as_ref(),
                ]
            })
            .flatten()
            .filter(|op| op.operation_id.as_deref().is_none_or(str::is_empty))
            .count();

        if missing_operation_ids > 0 {
            warnings.push(format!(
                "{missing_operation_ids} operations are missing operationId"
            ));
        }

        if auth_types.is_empty() {
            warnings.push("no explicit auth scheme detected".to_string());
        }

        Ok(ValidationReport {
            is_valid: errors.is_empty(),
            quality_score,
            errors,
            warnings,
            missing_fields,
            auth_types,
        })
    }

    pub fn score_quality(spec: &OpenAPI) -> u8 {
        let mut score: i32 = 100;

        if spec.info.title.trim().is_empty() {
            score -= 20;
        }
        if spec.info.version.trim().is_empty() {
            score -= 20;
        }
        if spec.paths.paths.is_empty() {
            score -= 40;
        }
        if spec.servers.is_empty() {
            score -= 10;
        }

        let total_operations = spec
            .paths
            .paths
            .values()
            .filter_map(|item| item.as_item())
            .map(|item| {
                [
                    item.get.as_ref(),
                    item.put.as_ref(),
                    item.post.as_ref(),
                    item.delete.as_ref(),
                    item.patch.as_ref(),
                    item.options.as_ref(),
                    item.head.as_ref(),
                    item.trace.as_ref(),
                ]
                .into_iter()
                .flatten()
                .count()
            })
            .sum::<usize>();

        if total_operations == 0 {
            score -= 20;
        }

        score.clamp(0, 100) as u8
    }

    pub fn check_required_fields(spec: &OpenAPI) -> Vec<String> {
        let mut missing = Vec::new();
        if spec.openapi.trim().is_empty() {
            missing.push("openapi".to_string());
        }
        if spec.info.title.trim().is_empty() {
            missing.push("info.title".to_string());
        }
        if spec.info.version.trim().is_empty() {
            missing.push("info.version".to_string());
        }
        if spec.paths.paths.is_empty() {
            missing.push("paths".to_string());
        }
        missing
    }

    pub fn detect_auth_type(spec: &OpenAPI) -> Vec<AuthType> {
        let mut auth_types = Vec::new();
        if let Some(components) = &spec.components {
            for scheme in components.security_schemes.values() {
                match &scheme {
                    openapiv3::ReferenceOr::Item(SecurityScheme::APIKey { .. }) => {
                        auth_types.push(AuthType::ApiKey);
                    }
                    openapiv3::ReferenceOr::Item(SecurityScheme::HTTP { scheme, .. }) => {
                        if scheme.eq_ignore_ascii_case("bearer") {
                            auth_types.push(AuthType::Bearer);
                        } else if scheme.eq_ignore_ascii_case("basic") {
                            auth_types.push(AuthType::Basic);
                        } else {
                            auth_types.push(AuthType::Unknown);
                        }
                    }
                    openapiv3::ReferenceOr::Item(SecurityScheme::OAuth2 { .. }) => {
                        auth_types.push(AuthType::OAuth2);
                    }
                    openapiv3::ReferenceOr::Item(SecurityScheme::OpenIDConnect { .. }) => {
                        auth_types.push(AuthType::OpenIdConnect);
                    }
                    openapiv3::ReferenceOr::Reference { .. } => {}
                }
            }
        }
        auth_types.sort_by_key(|v| format!("{v:?}"));
        auth_types.dedup();
        auth_types
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub quality_score: u8,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub missing_fields: Vec<String>,
    pub auth_types: Vec<AuthType>,
}
