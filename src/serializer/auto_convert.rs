//! Automatic JSON to Serializer conversion utilities
//!
//! This module provides utilities to automatically convert JSON to Serializer format
//! wherever possible to save tokens. It includes smart detection, conversion, and
//! fallback mechanisms.

use crate::serializer::{decode_default, encode_default};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Automatically convert any serializable value to Serializer format
/// Falls back to JSON if conversion fails
pub fn to_serializer_or_json<T: Serialize>(value: &T) -> String {
    // Try Serializer format first
    match encode_default(value) {
        Ok(serializer_str) => serializer_str,
        Err(_) => {
            // Fallback to JSON
            serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

/// Convert any serializable value to pretty Serializer format
/// Falls back to pretty JSON if conversion fails
pub fn to_serializer_or_json_pretty<T: Serialize + ?Sized>(value: &T) -> String {
    // Try Serializer format first (already pretty by default)
    match encode_default(value) {
        Ok(serializer_str) => serializer_str,
        Err(_) => {
            // Fallback to pretty JSON
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

/// Parse from either Serializer or JSON format automatically
pub fn from_serializer_or_json<T: for<'de> Deserialize<'de>>(input: &str) -> Result<T, String> {
    // Try Serializer format first
    if is_likely_serializer(input) {
        match decode_default::<T>(input) {
            Ok(value) => return Ok(value),
            Err(_) => {
                // Fall through to JSON
            }
        }
    }

    // Try JSON format
    serde_json::from_str(input).map_err(|e| format!("Failed to parse as Serializer or JSON: {}", e))
}

/// Convert JSON string to Serializer format
/// Returns the original JSON if conversion fails
pub fn json_to_serializer(json_str: &str) -> String {
    match serde_json::from_str::<Value>(json_str) {
        Ok(value) => to_serializer_or_json(&value),
        Err(_) => json_str.to_string(),
    }
}

/// Convert JSON Value to Serializer string
pub fn json_value_to_serializer(value: &Value) -> String {
    to_serializer_or_json(value)
}

/// Check if a string is likely Serializer format (not JSON)
pub fn is_likely_serializer(s: &str) -> bool {
    let trimmed = s.trim();

    // Empty or very short strings - ambiguous
    if trimmed.len() < 2 {
        return false;
    }

    // Definitely JSON if starts with { or [
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        return false;
    }

    // Likely Serializer if has key: value pattern
    if trimmed.contains(": ") || trimmed.contains(":\n") {
        return true;
    }

    // Likely Serializer if has array notation like items[3]:
    if trimmed.contains("[") && trimmed.contains("]:") {
        return true;
    }

    // Likely Serializer if has tabular notation like items[3]{id,name}:
    if trimmed.contains("{") && trimmed.contains("}:") {
        return true;
    }

    // Default to false (assume JSON)
    false
}

/// Calculate token savings from using Serializer vs JSON
pub fn calculate_savings(json_str: &str, serializer_str: &str) -> TokenSavings {
    let json_tokens = estimate_tokens(json_str);
    let serializer_tokens = estimate_tokens(serializer_str);

    let saved_tokens = json_tokens.saturating_sub(serializer_tokens);
    let saved_bytes = json_str.len().saturating_sub(serializer_str.len());

    let token_savings_percent = if json_tokens > 0 {
        (saved_tokens as f64 / json_tokens as f64) * 100.0
    } else {
        0.0
    };

    let byte_savings_percent = if json_str.len() > 0 {
        (saved_bytes as f64 / json_str.len() as f64) * 100.0
    } else {
        0.0
    };

    TokenSavings {
        json_tokens,
        serializer_tokens,
        saved_tokens,
        json_bytes: json_str.len(),
        serializer_bytes: serializer_str.len(),
        saved_bytes,
        token_savings_percent,
        byte_savings_percent,
    }
}

/// Token savings statistics
#[derive(Debug, Clone)]
pub struct TokenSavings {
    pub json_tokens: usize,
    pub serializer_tokens: usize,
    pub saved_tokens: usize,
    pub json_bytes: usize,
    pub serializer_bytes: usize,
    pub saved_bytes: usize,
    pub token_savings_percent: f64,
    pub byte_savings_percent: f64,
}

impl TokenSavings {
    pub fn format_summary(&self) -> String {
        format!(
            "Tokens: {} → {} (saved {} / {:.1}%) | Bytes: {} → {} (saved {} / {:.1}%)",
            self.json_tokens,
            self.serializer_tokens,
            self.saved_tokens,
            self.token_savings_percent,
            self.json_bytes,
            self.serializer_bytes,
            self.saved_bytes,
            self.byte_savings_percent
        )
    }
}

/// Estimate token count (rough approximation: ~4 chars per token)
fn estimate_tokens(s: &str) -> usize {
    // Simple heuristic: count words, punctuation, and divide by average
    let words = s.split_whitespace().count();
    let punctuation = s.chars().filter(|c| c.is_ascii_punctuation()).count();
    let numbers = s.chars().filter(|c| c.is_ascii_digit()).count() / 3; // Numbers often tokenize differently

    // Rough estimate
    (words + punctuation + numbers).max(s.len() / 4)
}

/// Wrapper for tool call arguments that automatically uses Serializer format
#[derive(Debug, Clone)]
pub struct SerializerToolArgs {
    inner: Value,
}

impl SerializerToolArgs {
    pub fn new(value: Value) -> Self {
        Self { inner: value }
    }

    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        Ok(Self {
            inner: serde_json::from_str(json)?,
        })
    }

    pub fn to_serializer(&self) -> String {
        json_value_to_serializer(&self.inner)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn value(&self) -> &Value {
        &self.inner
    }
}

impl std::fmt::Display for SerializerToolArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_serializer())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_to_serializer_simple() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let serializer = json_to_serializer(json);

        assert!(serializer.contains("name: Alice"));
        assert!(serializer.contains("age: 30"));
        assert!(!serializer.contains("{"));
    }

    #[test]
    fn test_json_to_serializer_array() {
        let json = r#"{"tags": ["a", "b", "c"]}"#;
        let serializer = json_to_serializer(json);

        assert!(serializer.contains("tags[3]:"));
        assert!(serializer.contains("a,b,c"));
    }

    #[test]
    fn test_is_likely_serializer() {
        assert!(is_likely_serializer("name: Alice\nage: 30"));
        assert!(is_likely_serializer("items[3]: a,b,c"));
        assert!(is_likely_serializer(
            "users[2]{id,name}:\n  1,Alice\n  2,Bob"
        ));

        assert!(!is_likely_serializer(r#"{"name": "Alice"}"#));
        assert!(!is_likely_serializer(r#"["a", "b", "c"]"#));
    }

    #[test]
    fn test_from_serializer_or_json() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct User {
            name: String,
            age: u32,
        }

        // Parse from Serializer
        let serializer = "name: Alice\nage: 30";
        let user: User = from_serializer_or_json(serializer).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);

        // Parse from JSON
        let json = r#"{"name": "Bob", "age": 25}"#;
        let user: User = from_serializer_or_json(json).unwrap();
        assert_eq!(user.name, "Bob");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_calculate_savings() {
        let json = r#"{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#;
        let serializer = "users[2]{id,name}:\n  1,Alice\n  2,Bob";

        let savings = calculate_savings(json, serializer);

        assert!(savings.saved_tokens > 0);
        assert!(savings.saved_bytes > 0);
        assert!(savings.token_savings_percent > 0.0);
    }

    #[test]
    fn test_serializer_tool_args() {
        let value = json!({"path": "test.txt", "content": "hello"});
        let args = SerializerToolArgs::new(value);

        let serializer = args.to_serializer();
        assert!(serializer.contains("path: test.txt"));
        assert!(serializer.contains("content: hello"));

        let json = args.to_json();
        assert!(json.contains("\"path\""));
        assert!(json.contains("\"content\""));
    }

    #[test]
    fn test_to_serializer_or_json_fallback() {
        // Valid struct should convert to Serializer
        #[derive(Serialize)]
        struct Valid {
            name: String,
            count: u32,
        }

        let valid = Valid {
            name: "test".to_string(),
            count: 42,
        };

        let result = to_serializer_or_json(&valid);
        assert!(result.contains("name: test"));
        assert!(result.contains("count: 42"));
    }
}
