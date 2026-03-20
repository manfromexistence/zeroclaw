//! Quick test binary for RLM token-saving system
//!
//! Usage: cargo run --bin test-rlm

use zeroclaw::token::rlm::RLM;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("MISTRAL_API_KEY")
        .or_else(|_| env::var("GROQ_API_KEY"))
        .expect("Set MISTRAL_API_KEY or GROQ_API_KEY environment variable");

    println!("🚀 Testing RLM (Recursive Language Model)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create RLM instance with Mistral models
    let rlm = RLM::new(
        api_key,
        "mistral-large-latest".to_string(),
    )
    .with_fast_model("mistral-small-latest".to_string())
    .with_max_iterations(10);

    // Test with a simple document
    let context = r#"
    ZeroClaw is a Rust-based AI agent framework built for speed and efficiency.
    It features:
    - 215+ search engines via metasearch integration
    - Token-saving systems including RLM (37% savings)
    - Multi-provider support (OpenAI, Anthropic, Mistral, Groq, etc.)
    - Hardware integration (STM32, Arduino, Raspberry Pi)
    - Multi-channel support (Telegram, Discord, Slack, WhatsApp)
    
    The project is written in Rust using edition 2024 and focuses on:
    - Zero overhead
    - Zero compromise
    - 100% Rust implementation
    - Fastest, smallest AI assistant
    "#;

    let query = "What are the main features of ZeroClaw?";

    println!("📝 Query: {}", query);
    println!("📄 Context size: {} chars\n", context.len());

    // Execute RLM
    println!("⚙️  Processing with RLM...\n");
    
    match rlm.complete(query, context).await {
        Ok((answer, stats)) => {
            println!("✅ Answer:");
            println!("{}\n", answer);
            
            println!("📊 Statistics:");
            println!("  • LLM calls: {}", stats.llm_calls);
            println!("  • Iterations: {}", stats.iterations);
            println!("  • Time: {:.2}s", stats.elapsed_ms as f64 / 1000.0);
            println!("  • Cache hit rate: {:.1}%", stats.cache_hit_rate());
            println!("  • Cost savings: {:.1}%", stats.cost_savings());
            println!("  • Fast model calls: {}", stats.fast_model_calls);
            println!("  • Smart model calls: {}", stats.smart_model_calls);
            
            println!("\n✨ RLM integration successful!");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
