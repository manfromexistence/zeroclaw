//! Instructions for AI models to use Serializer format for tool calling
//!
//! This module provides the system instructions that tell AI models to use
//! the compact Serializer format instead of JSON for tool calls, saving tokens.

/// System instruction for using Serializer format for tool calls
pub const SERIALIZER_TOOL_INSTRUCTION: &str = r#"
# Tool Call Format - Use Serializer (Not JSON)

For ALL tool calls, use the compact Serializer format instead of JSON to save tokens.

## Serializer Format Rules

### Simple Objects
Instead of JSON:
```json
{"name": "Alice", "age": 30, "active": true}
```

Use Serializer:
```
name: Alice
age: 30
active: true
```

### Arrays (Inline)
Instead of JSON:
```json
{"tags": ["reading", "gaming", "coding"]}
```

Use Serializer:
```
tags[3]: reading,gaming,coding
```

### Tabular Arrays (Objects with same keys)
Instead of JSON:
```json
{
  "users": [
    {"id": 1, "name": "Alice", "role": "admin"},
    {"id": 2, "name": "Bob", "role": "user"}
  ]
}
```

Use Serializer:
```
users[2]{id,name,role}:
  1,Alice,admin
  2,Bob,user
```

### Nested Objects
Instead of JSON:
```json
{
  "config": {
    "server": {
      "host": "localhost",
      "port": 8080
    }
  }
}
```

Use Serializer:
```
config:
  server:
    host: localhost
    port: 8080
```

### Mixed Types
Combine formats as needed:
```
user:
  name: Alice
  tags[3]: dev,rust,ai
  settings:
    theme: dark
    notifications: true
```

## Tool Call Examples

### Example 1: File Write
Instead of:
```json
{
  "tool": "write_file",
  "path": "src/main.rs",
  "content": "fn main() {\n    println!(\"Hello\");\n}"
}
```

Use:
```
tool: write_file
path: src/main.rs
content: fn main() {\n    println!("Hello");\n}
```

### Example 2: Search with Filters
Instead of:
```json
{
  "tool": "search",
  "query": "rust async",
  "filters": {
    "language": "rust",
    "stars": ">100"
  },
  "limit": 10
}
```

Use:
```
tool: search
query: rust async
filters:
  language: rust
  stars: >100
limit: 10
```

### Example 3: Batch Operations
Instead of:
```json
{
  "tool": "batch_update",
  "items": [
    {"id": 1, "status": "done"},
    {"id": 2, "status": "pending"},
    {"id": 3, "status": "done"}
  ]
}
```

Use:
```
tool: batch_update
items[3]{id,status}:
  1,done
  2,pending
  3,done
```

## Fallback to JSON

If Serializer format fails or is rejected, automatically fall back to standard JSON format.

## Benefits

- **18-40% token savings** on average
- Faster parsing for simple structures
- More human-readable
- Maintains full data fidelity

## Important Notes

1. Always use Serializer format FIRST
2. Only use JSON if Serializer fails
3. Strings with special characters need quotes: "hello\nworld"
4. Numbers, booleans, null are written as-is: 42, true, false, null
5. Arrays use [count] notation: items[5]
6. Tabular arrays use {keys} notation: users[3]{id,name}
"#;

/// Get the serializer instruction to inject into system prompts
pub fn get_serializer_instruction() -> &'static str {
    SERIALIZER_TOOL_INSTRUCTION
}

/// Check if a string looks like serializer format
pub fn is_serializer_format(s: &str) -> bool {
    // Simple heuristic: serializer format typically has "key: value" patterns
    // and doesn't start with { or [
    let trimmed = s.trim();
    !trimmed.starts_with('{') 
        && !trimmed.starts_with('[')
        && (trimmed.contains(": ") || trimmed.contains(":\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_serializer_format() {
        assert!(is_serializer_format("name: Alice\nage: 30"));
        assert!(is_serializer_format("tool: search\nquery: test"));
        assert!(!is_serializer_format(r#"{"name": "Alice"}"#));
        assert!(!is_serializer_format(r#"["a", "b", "c"]"#));
    }
}
