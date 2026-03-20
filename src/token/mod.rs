//! Token optimization and cost reduction system
//!
//! This module provides various token-saving techniques to reduce LLM API costs:
//!
//! ## Key Components
//!
//! - **RLM (Recursive Language Model)** - 37% token savings on large documents
//!
//! ## Usage
//!
//! ```rust
//! use crate::token::rlm::RLM;
//!
//! // Use RLM for large document processing (37% token savings)
//! let rlm = RLM::new(api_key, model);
//! let (answer, stats) = rlm.complete(query, large_document).await?;
//! println!("Saved {}% tokens", stats.cost_savings());
//! ```

#[path = "rlm/lib.rs"]
pub mod rlm;

// TODO: The following modules require dx_core types to be integrated
// Uncomment after creating a minimal dx_core compatibility layer
/*
#[path = "compaction/lib.rs"]
pub mod compaction;

#[path = "prompt-compress/lib.rs"]
pub mod prompt_compress;

#[path = "context-pruner/lib.rs"]
pub mod context_pruner;

#[path = "dedup/lib.rs"]
pub mod dedup;

#[path = "semantic-cache/lib.rs"]
pub mod semantic_cache;

#[path = "prefix-cache/lib.rs"]
pub mod prefix_cache;

#[path = "response-cache/lib.rs"]
pub mod response_cache;

#[path = "token-budget/lib.rs"]
pub mod token_budget;

#[path = "whitespace-normalize/lib.rs"]
pub mod whitespace_normalize;
*/

// Re-export key types
pub use rlm::{RLM, RLMStats};
