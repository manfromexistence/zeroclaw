//! Helper functions for using Serializer format in agent operations
//!
//! This module provides utilities to automatically use Serializer format
//! for tool calls, responses, and data exchange to maximize token savings.

use crate::serializer::{
    TokenSavings, calculate_savings, from_serializer_or_json, is_likely_serializer,
    json_to_serializer, to_serializer_or_json, to_serializer_or_json_pretty,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Format tool call arguments using Serializer format
pub fn format_tool_args<T: Serialize>(args: &T) -> String {
    to_serializer_or_json(args)
}

/// Format tool response using Serializer format
pub fn format_tool_response<T: Serialize>(response: &T) -> String {
    to_serializer_or_json_pretty(response)
}

/// Parse tool call arguments from either Serializer or JSON
pub fn parse_tool_args<'a, T: Deserialize<'a>>(input: &'a str) -> Result<T, String> {
    from_serializer_or_json(input)
}

/// Convert JSON tool call to Serializer format
pub fn convert_json_tool_call(json_str: &str) -> String {
    json_to_serializer(json_str)
}

/// Format context data for prompt using Serializer format
pub fn format_context_data<T: Serialize>(data: &T) -> String {
    to_serializer_or_json_pretty(data)
}

/// Format memory entries using Serializer format
pub fn format_memory_entries(entries: &[MemoryEntry]) -> String {
    to_serializer_or_json_pretty(entries)
}

/// Format skill data using Serializer format
pub fn format_skill_data<T: Serialize>(skill_data: &T) -> String {
    to_serializer_or_json_pretty(skill_data)
}

/// Check if input is Serializer format and report savings
pub fn analyze_format(input: &str) -> FormatAnalysis {
    let is_serializer = is_likely_serializer(input);

    if is_serializer {
        FormatAnalysis {
            format: DataFormat::Serializer,
            savings: None,
        }
    } else {
        // Try to convert and calculate savings
        let serializer_version = json_to_serializer(input);
        let savings = if serializer_version != input {
            Some(calculate_savings(input, &serializer_version))
        } else {
            None
        };

        FormatAnalysis {
            format: DataFormat::Json,
            savings,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Serializer,
    Json,
}

#[derive(Debug, Clone)]
pub struct FormatAnalysis {
    pub format: DataFormat,
    pub savings: Option<TokenSavings>,
}

impl FormatAnalysis {
    pub fn would_save_tokens(&self) -> bool {
        self.savings.as_ref().map_or(false, |s| s.saved_tokens > 0)
    }

    pub fn savings_summary(&self) -> Option<String> {
        self.savings.as_ref().map(|s| s.format_summary())
    }
}

/// Wrapper for agent messages that automatically uses Serializer format
#[derive(Debug, Clone)]
pub struct SerializerMessage {
    pub role: String,
    pub content: String,
    pub metadata: Option<Value>,
}

impl SerializerMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn to_serializer(&self) -> String {
        let mut parts = vec![
            format!("role: {}", self.role),
            format!("content: {}", self.content),
        ];

        if let Some(ref meta) = self.metadata {
            parts.push(format!("metadata:\n{}", to_serializer_or_json_pretty(meta)));
        }

        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_tool_args() {
        #[derive(Serialize)]
        struct Args {
            path: String,
            content: String,
        }

        let args = Args {
            path: "test.txt".to_string(),
            content: "hello world".to_string(),
        };

        let formatted = format_tool_args(&args);
        assert!(formatted.contains("path: test.txt"));
        assert!(formatted.contains("content: hello world"));
    }

    #[test]
    fn test_parse_tool_args() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Args {
            path: String,
            content: String,
        }

        // Parse from Serializer
        let serializer = "path: test.txt\ncontent: hello world";
        let args: Args = parse_tool_args(serializer).unwrap();
        assert_eq!(args.path, "test.txt");
        assert_eq!(args.content, "hello world");

        // Parse from JSON
        let json = r#"{"path": "test.txt", "content": "hello world"}"#;
        let args: Args = parse_tool_args(json).unwrap();
        assert_eq!(args.path, "test.txt");
        assert_eq!(args.content, "hello world");
    }

    #[test]
    fn test_analyze_format() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let analysis = analyze_format(json);

        assert_eq!(analysis.format, DataFormat::Json);
        assert!(analysis.would_save_tokens());
        assert!(analysis.savings_summary().is_some());
    }

    #[test]
    fn test_serializer_message() {
        let msg = SerializerMessage::new("user", "Hello, world!")
            .with_metadata(json!({"timestamp": 1234567890}));

        let serializer = msg.to_serializer();
        assert!(serializer.contains("role: user"));
        assert!(serializer.contains("content: Hello, world!"));
        assert!(serializer.contains("metadata:"));
        assert!(serializer.contains("timestamp: 1234567890"));
    }
}
