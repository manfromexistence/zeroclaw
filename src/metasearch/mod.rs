//! # Metasearch - 215+ Search Engines
//!
//! Privacy-respecting metasearch engine with support for 215+ search engines.
//! Inspired by SearXNG, fully rewritten in Rust for maximum performance.

pub mod category;
pub mod config;
pub mod engine;
pub mod engines;
pub mod error;
pub mod query;
pub mod ranking;
pub mod result;

pub use category::SearchCategory;
pub use config::Settings;
pub use engine::{EngineMetadata, SearchEngine};
pub use engines::EngineRegistry;
pub use error::MetasearchError;
pub use query::SearchQuery;
pub use ranking::ResultAggregator;
pub use result::{SearchResponse, SearchResult};
