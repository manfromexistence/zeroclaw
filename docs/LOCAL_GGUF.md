# Local GGUF Model Support

DX-Agent supports running local GGUF models directly using llama.cpp bindings, without requiring external servers like Ollama.

## Features

- **Direct GGUF inference** - Load and run quantized models locally
- **Persistent KV cache** - Context is preserved across conversation turns for efficiency
- **Streaming support** - Token-by-token generation
- **Context window management** - Automatic eviction of old turns when approaching limit
- **GPU acceleration** - Automatic GPU offloading when available
- **Flash attention** - Uses flash attention when supported by the model

## Installation

Build with the `local-gguf` feature:

```bash
cargo build --release --features local-gguf
```

## Configuration

### Model Path

Set the model path via environment variable:

```bash
export DX_MODEL_PATH=/path/to/your/model.gguf
```

Or use the default path: `models/model.gguf`

### Recommended Models

- **Qwen 2.5 Coder** (0.5B-14B) - Excellent for code generation
- **Llama 3.2** (1B-3B) - Fast and capable
- **Phi-3** (3.8B) - Microsoft's efficient model
- **DeepSeek Coder** (1.3B-6.7B) - Strong coding performance

Use Q4_K_M or Q5_K_M quantization for best balance of speed and quality.

## Usage

### Environment Variables

- `DX_MODEL_PATH` - Path to GGUF model file (default: `models/model.gguf`)

### Configuration Constants

Edit `src/providers/local_gguf.rs` to adjust:

- `INFERENCE_CONTEXT_TOKENS` - Context window size (default: 32,768)
- `MAX_GENERATION_TOKENS` - Max tokens per response (default: 4,096)
- `SAMPLER_TEMPERATURE` - Sampling temperature (default: 0.7)
- `SAMPLER_TOP_P` - Nucleus sampling threshold (default: 0.92)
- `SAMPLER_TOP_K` - Top-K sampling (default: 40)

## Performance

### Memory Requirements

- **1B model (Q4)**: ~1GB RAM
- **3B model (Q4)**: ~2.5GB RAM
- **7B model (Q4)**: ~5GB RAM
- **13B model (Q4)**: ~9GB RAM

### GPU Acceleration

The provider automatically offloads layers to GPU when available. Set `n_gpu_layers` in the code to control offloading (default: 999 = all layers).

### Thread Count

Automatically uses `physical_cores - 1` threads for optimal performance.

## Architecture

### Persistent Context

Unlike stateless inference, the local GGUF provider maintains:

- **KV cache** - Reused across turns, only new tokens are evaluated
- **Conversation history** - Full chat context preserved
- **Token tracking** - For sampler penalty calculation

### Context Eviction

When token usage exceeds 85% of context window:
1. Oldest user/assistant pairs are removed
2. KV cache is cleared
3. Remaining history is re-encoded on next turn

This prevents silent truncation and maintains conversation quality.

## Comparison with Ollama

| Feature | Local GGUF | Ollama |
|---------|-----------|--------|
| Setup | Single binary | Separate server |
| Memory | Lower (no server overhead) | Higher |
| Startup | Instant | Server must be running |
| Context | Persistent across calls | Per-request |
| Control | Full (edit code) | Limited (API only) |

## Troubleshooting

### Model fails to load

- Check `DX_MODEL_PATH` is correct
- Verify GGUF file is not corrupted
- Ensure sufficient RAM available

### Out of memory during generation

- Reduce `INFERENCE_CONTEXT_TOKENS`
- Use smaller model or higher quantization (Q4 instead of Q5)
- Enable GPU offloading

### Slow generation

- Check GPU is being used (`n_gpu_layers > 0`)
- Verify thread count is optimal
- Consider using flash attention compatible model

## Example

```rust
use agent::providers::local_gguf::LocalGgufProvider;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = LocalGgufProvider::new();
    
    // Initialize (loads model)
    provider.initialize().await?;
    
    // Generate streaming response
    let cancel = CancellationToken::new();
    provider.generate_stream(
        "Write a hello world in Rust",
        cancel,
        |token| print!("{}", token),
    ).await?;
    
    Ok(())
}
```

## Future Enhancements

- [ ] Dynamic model loading (switch models without restart)
- [ ] Multi-model support (load multiple models)
- [ ] Configurable system prompt
- [ ] LoRA adapter support
- [ ] Speculative decoding
- [ ] Batch inference for multiple requests
