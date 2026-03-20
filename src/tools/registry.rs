//! Tool registry for managing available tools.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

use crate::tools::definition::{Tool, ToolCall, ToolDefinition, ToolResult};

/// Registry that holds all available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a new tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.definition().name.clone();
        self.tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// List all tool definitions (for LLM function calling)
    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .filter(|t| t.is_available())
            .map(|t| t.definition())
            .collect()
    }

    /// Execute a tool call
    pub async fn execute(&self, call: ToolCall) -> Result<ToolResult> {
        let tool = self
            .tools
            .get(&call.name)
            .ok_or_else(|| anyhow::anyhow!("Unknown tool: {}", call.name))?;

        if !tool.is_available() {
            return Ok(ToolResult::error(
                call.id.clone(),
                format!("Tool '{}' is not available", call.name),
            ));
        }

        tool.execute(call).await
    }

    /// Number of registered tools
    pub fn count(&self) -> usize {
        self.tools.len()
    }

    /// Get tools by category
    pub fn by_category(&self, category: &str) -> Vec<ToolDefinition> {
        self.tools
            .values()
            .filter(|t| t.definition().category == category)
            .map(|t| t.definition())
            .collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let mut r = Self::new();
        r.register_all();
        r
    }
}

impl ToolRegistry {
    /// Register all built-in tools (only those that exist in src/tools).
    pub fn register_all(&mut self) {
        // Note: Only registering tools that are actually implemented in src/tools/
        // Many tools from the original tools/ folder are stubs and not yet integrated
        
        // TODO: Implement and register the 42 merged tools once their dependencies are resolved
        // For now, this registry is a placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::definition::*;
    use async_trait::async_trait;

    struct EchoTool;

    #[async_trait]
    impl Tool for EchoTool {
        fn definition(&self) -> ToolDefinition {
            ToolDefinition {
                name: "echo".into(),
                description: "Echo input back".into(),
                parameters: vec![ToolParameter {
                    name: "text".into(),
                    description: "Text to echo".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                    enum_values: None,
                }],
                category: "test".into(),
                requires_confirmation: false,
            }
        }

        async fn execute(&self, call: ToolCall) -> Result<ToolResult> {
            let text = call.arguments.get("text").and_then(|v| v.as_str()).unwrap_or("(empty)");
            Ok(ToolResult::success(call.id, text.to_string()))
        }
    }

    #[tokio::test]
    async fn test_registry() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(EchoTool));

        assert_eq!(registry.count(), 1);
        assert_eq!(registry.definitions().len(), 1);

        let call = ToolCall {
            id: "1".into(),
            name: "echo".into(),
            arguments: serde_json::json!({"text": "hello"}),
        };

        let result = registry.execute(call).await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "hello");
    }
}
