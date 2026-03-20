# Serializer Format Integration

DX-Agent uses a compact Serializer format (based on TOON - Token-Oriented Object Notation) for tool calling and data exchange with AI models, achieving 18-40% token savings compared to JSON.

## Overview

The Serializer format is automatically used for:
- Tool call parameters
- Tool call responses
- Structured data in prompts
- Configuration data passed to models

JSON is used as a fallback if Serializer parsing fails.

## Format Comparison

### Simple Objects

**JSON (40 bytes, 16 tokens):**
```json
{
  "name": "Alice",
  "age": 30,
  "active": true
}
```

**Serializer (28 bytes, 13 tokens) - 30% savings:**
```
name: Alice
age: 30
active: true
```

### Arrays

**JSON (38 bytes, 13 tokens):**
```json
{
  "tags": ["reading", "gaming", "coding"]
}
```

**Serializer (28 bytes, 10 tokens) - 23% savings:**
```
tags[3]: reading,gaming,coding
```

### Tabular Data

**JSON (120 bytes, 35 tokens):**
```json
{
  "users": [
    {"id": 1, "name": "Alice", "role": "admin"},
    {"id": 2, "name": "Bob", "role": "user"}
  ]
}
```

**Serializer (58 bytes, 21 tokens) - 40% savings:**
```
users[2]{id,name,role}:
  1,Alice,admin
  2,Bob,user
```

## Integration Points

### 1. System Prompts

The serializer instruction is automatically injected into system prompts for all AI models:

```rust
use agent::agent::get_serializer_instruction;

let system_prompt = format!(
    "{}\n\n{}",
    base_system_prompt,
    get_serializer_instruction()
);
```

### 2. Tool Call Parsing

The agent automatically detects and parses both formats:

```rust
use agent::serializer::{decode_default, ToonError};
use agent::agent::is_serializer_format;

fn parse_tool_call(input: &str) -> Result<ToolCall> {
    if is_serializer_format(input) {
        // Try serializer format first
        match decode_default::<ToolCall>(input) {
            Ok(call) => Ok(call),
            Err(_) => {
                // Fallback to JSON
                serde_json::from_str(input)
            }
        }
    } else {
        // Parse as JSON
        serde_json::from_str(input)
    }
}
```

### 3. Tool Response Encoding

Tool responses are encoded in Serializer format:

```rust
use agent::serializer::encode_default;

fn format_tool_response(result: &ToolResult) -> String {
    // Try serializer format first
    encode_default(result)
        .unwrap_or_else(|_| {
            // Fallback to JSON
            serde_json::to_string_pretty(result).unwrap()
        })
}
```

## Usage Examples

### Example 1: File Operations

**Tool Call (Serializer):**
```
tool: write_file
path: src/config.rs
content: pub const VERSION: &str = "1.0.0";
mode: 0o644
```

**Response (Serializer):**
```
success: true
bytes_written: 35
path: src/config.rs
```

### Example 2: Search Operations

**Tool Call (Serializer):**
```
tool: search_code
query: async fn
filters:
  language: rust
  path: src/**/*.rs
limit: 10
```

**Response (Serializer):**
```
results[3]{file,line,match}:
  src/main.rs,42,async fn main()
  src/lib.rs,15,async fn process()
  src/agent.rs,88,async fn run()
total: 3
```

### Example 3: Batch Operations

**Tool Call (Serializer):**
```
tool: batch_update
operations[3]{type,target,value}:
  set,config.debug,true
  append,config.features,serializer
  delete,config.deprecated,null
```

**Response (Serializer):**
```
success: true
applied: 3
failed: 0
```

## API Reference

### Encoding

```rust
use agent::serializer::{encode, encode_default, EncodeOptions};

// Default encoding
let serializer_str = encode_default(&data)?;

// Custom options
let opts = EncodeOptions::new()
    .with_delimiter(Delimiter::Pipe)
    .with_indent(Indent::Spaces(4));
let serializer_str = encode(&data, &opts)?;
```

### Decoding

```rust
use agent::serializer::{decode, decode_default, DecodeOptions};

// Default decoding (strict)
let data: MyStruct = decode_default(&serializer_str)?;

// Custom options
let opts = DecodeOptions::new()
    .with_strict(false)
    .with_coerce_types(true);
let data: MyStruct = decode(&serializer_str, &opts)?;
```

### Detection

```rust
use agent::agent::is_serializer_format;

if is_serializer_format(input) {
    // Parse as serializer
} else {
    // Parse as JSON
}
```

## Configuration

### Enable/Disable Serializer

Set in config.toml:

```toml
[agent]
use_serializer_format = true  # Default: true
fallback_to_json = true        # Default: true
```

### Model-Specific Settings

Some models may work better with JSON. Configure per-model:

```toml
[providers.openai]
use_serializer = true

[providers.anthropic]
use_serializer = true

[providers.local]
use_serializer = false  # Disable for local models if needed
```

## Benefits

1. **Token Savings**: 18-40% reduction in token usage
2. **Faster Parsing**: Simpler format = faster processing
3. **Human Readable**: Easier to debug and understand
4. **Backward Compatible**: JSON fallback ensures compatibility
5. **Type Safe**: Full serde integration maintains type safety

## Limitations

1. **Complex Nested Structures**: Very deep nesting may be more verbose than JSON
2. **Binary Data**: Not suitable for binary data (use base64 in JSON fallback)
3. **Model Support**: Some models may not understand the format initially (fallback handles this)

## Troubleshooting

### Model Not Using Serializer

If the model continues using JSON:

1. Check system prompt includes serializer instructions
2. Verify `use_serializer_format = true` in config
3. Try few-shot examples in the prompt
4. Some models need explicit examples to learn the format

### Parse Errors

If serializer parsing fails:

1. Check the error message for syntax issues
2. Verify array lengths match: `items[3]: a,b,c` (3 items)
3. Ensure proper indentation (2 spaces default)
4. JSON fallback will activate automatically

### Performance Issues

If serializer encoding/decoding is slow:

1. Use `encode_default` for simple cases
2. Disable strict validation: `DecodeOptions::new().with_strict(false)`
3. Consider JSON for very large payloads (>10KB)

## Future Enhancements

- [ ] Streaming serializer parser for large responses
- [ ] Binary format variant for maximum compression
- [ ] Model fine-tuning data for better serializer adoption
- [ ] Automatic format selection based on data structure
- [ ] Compression for repeated patterns
