//! Registry for managing OpenAPI specs and generated tools.

use super::auth::AuthProvider;
use super::executor::OpenApiTool;
use super::spec::OpenApiSpec;
use crate::tools::traits::Tool;
use anyhow::{Context, Result};
use openapiv3::ReferenceOr;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Registry of loaded OpenAPI specs and their generated tools.
pub struct OpenApiRegistry {
    specs: RwLock<HashMap<String, Arc<OpenApiSpec>>>,
    tools: RwLock<HashMap<String, Arc<OpenApiTool>>>,
    auth_providers: RwLock<HashMap<String, Arc<dyn AuthProvider>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryIndex {
    specs: Vec<SpecEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpecEntry {
    id: String,
    provider: String,
    service: String,
    version: String,
    tier: String,
    quality_score: u8,
    tool_count: usize,
}

impl OpenApiRegistry {
    pub fn new() -> Self {
        Self {
            specs: RwLock::new(HashMap::new()),
            tools: RwLock::new(HashMap::new()),
            auth_providers: RwLock::new(HashMap::new()),
        }
    }

    /// Load specs from a registry.json file.
    pub fn load_from_disk(&self, registry_path: &Path) -> Result<()> {
        if !registry_path.exists() {
            tracing::warn!(
                "Registry file not found: {}. Run 'zeroclaw openapi harvest' first.",
                registry_path.display()
            );
            return Ok(());
        }

        let contents = std::fs::read_to_string(registry_path)
            .with_context(|| format!("failed to read registry {}", registry_path.display()))?;

        let registry: serde_json::Value = serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse registry {}", registry_path.display()))?;

        let specs_array = registry
            .get("specs")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("registry missing 'specs' array"))?;

        let specs_dir = registry_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid registry path"))?;

        let mut loaded_count = 0;
        let mut failed_count = 0;

        for spec_entry in specs_array {
            let spec_id = spec_entry
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let spec_path = spec_entry
                .get("path")
                .and_then(|v| v.as_str())
                .map(|p| specs_dir.join(p));

            if let Some(path) = spec_path {
                match OpenApiSpec::from_file(&path) {
                    Ok(spec) => {
                        self.register_spec(spec_id.to_string(), spec)?;
                        loaded_count += 1;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load spec {}: {}", spec_id, e);
                        failed_count += 1;
                    }
                }
            }
        }

        tracing::info!(
            "Loaded {} OpenAPI specs ({} failed)",
            loaded_count,
            failed_count
        );

        Ok(())
    }

    /// Register a spec and generate tools for all operations.
    pub fn register_spec(&self, spec_id: String, spec: OpenApiSpec) -> Result<()> {
        let spec_arc = Arc::new(spec);

        // Get or create auth provider for this spec
        let auth_provider = self.get_or_create_auth_provider(&spec_id, &spec_arc)?;

        // Generate tools for all operations
        let tools = self.create_tools_for_spec(&spec_arc, auth_provider)?;

        // Register spec
        self.specs.write().insert(spec_id.clone(), spec_arc);

        // Register tools
        let mut tools_map = self.tools.write();
        for tool in tools {
            tools_map.insert(tool.name().to_string(), tool);
        }

        Ok(())
    }

    fn get_or_create_auth_provider(
        &self,
        spec_id: &str,
        spec: &OpenApiSpec,
    ) -> Result<Option<Arc<dyn AuthProvider>>> {
        // Check if auth provider already exists
        if let Some(provider) = self.auth_providers.read().get(spec_id) {
            return Ok(Some(Arc::clone(provider)));
        }

        // For now, use NoAuth. In production, this would:
        // 1. Check config for auth credentials
        // 2. Load from secrets storage
        // 3. Create appropriate auth provider
        match spec.auth.auth_type {
            super::spec::AuthType::None => Ok(None),
            _ => {
                // Default to no auth for now
                // TODO: Implement auth provider creation from config
                Ok(None)
            }
        }
    }

    fn create_tools_for_spec(
        &self,
        spec: &Arc<OpenApiSpec>,
        auth_provider: Option<Arc<dyn AuthProvider>>,
    ) -> Result<Vec<Arc<OpenApiTool>>> {
        let mut tools = Vec::new();

        for (path, path_item_ref) in &spec.spec.paths.paths {
            if let ReferenceOr::Item(path_item) = path_item_ref {
                // Process each HTTP method
                let operations = vec![
                    ("get", &path_item.get),
                    ("post", &path_item.post),
                    ("put", &path_item.put),
                    ("delete", &path_item.delete),
                    ("patch", &path_item.patch),
                    ("head", &path_item.head),
                    ("options", &path_item.options),
                    ("trace", &path_item.trace),
                ];

                for (method, operation_opt) in operations {
                    if let Some(operation) = operation_opt {
                        // Use operationId or generate one
                        let operation_id = operation
                            .operation_id
                            .clone()
                            .unwrap_or_else(|| generate_operation_id(method, path));

                        let tool = Arc::new(OpenApiTool::new(
                            Arc::clone(spec),
                            operation_id,
                            path.clone(),
                            method.to_string(),
                            operation.clone(),
                            auth_provider.clone(),
                        ));

                        tools.push(tool);
                    }
                }
            }
        }

        Ok(tools)
    }

    /// Get a tool by name.
    pub fn get_tool(&self, name: &str) -> Option<Arc<OpenApiTool>> {
        self.tools.read().get(name).cloned()
    }

    /// List all tool names.
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.read().keys().cloned().collect()
    }

    /// List all spec IDs.
    pub fn list_specs(&self) -> Vec<String> {
        self.specs.read().keys().cloned().collect()
    }

    /// Get tools for a specific spec.
    pub fn get_tools_for_spec(&self, spec_id: &str) -> Vec<String> {
        let specs = self.specs.read();
        if let Some(_spec) = specs.get(spec_id) {
            let tools = self.tools.read();
            tools
                .keys()
                .filter(|name| name.starts_with(&format!("{}_", spec_id)))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Search tools by keyword.
    pub fn search_tools(&self, query: &str) -> Vec<String> {
        let query_lower = query.to_lowercase();
        let tools = self.tools.read();

        tools
            .iter()
            .filter(|(name, tool)| {
                name.to_lowercase().contains(&query_lower)
                    || tool.description().to_lowercase().contains(&query_lower)
            })
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Get all tools as trait objects.
    pub fn get_all_tools(&self) -> Vec<Arc<dyn crate::tools::traits::Tool>> {
        self.tools
            .read()
            .values()
            .map(|tool| Arc::clone(tool) as Arc<dyn crate::tools::traits::Tool>)
            .collect()
    }

    /// Get spec metadata.
    pub fn get_spec(&self, spec_id: &str) -> Option<Arc<OpenApiSpec>> {
        self.specs.read().get(spec_id).cloned()
    }

    /// Get tool count.
    pub fn tool_count(&self) -> usize {
        self.tools.read().len()
    }

    /// Get spec count.
    pub fn spec_count(&self) -> usize {
        self.specs.read().len()
    }
}

impl Default for OpenApiRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_operation_id(method: &str, path: &str) -> String {
    // Convert /users/{id}/posts to users_id_posts
    let sanitized = path
        .trim_start_matches('/')
        .replace('/', "_")
        .replace('{', "")
        .replace('}', "")
        .replace('-', "_");

    format!("{}_{}", method, sanitized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_operation_id_formats_correctly() {
        assert_eq!(generate_operation_id("get", "/users/{id}"), "get_users_id");
        assert_eq!(
            generate_operation_id("post", "/users/{id}/posts"),
            "post_users_id_posts"
        );
        assert_eq!(
            generate_operation_id("delete", "/api/v1/items"),
            "delete_api_v1_items"
        );
    }

    #[test]
    fn registry_starts_empty() {
        let registry = OpenApiRegistry::new();
        assert_eq!(registry.tool_count(), 0);
        assert_eq!(registry.spec_count(), 0);
    }
}
