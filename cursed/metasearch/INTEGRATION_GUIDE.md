# 🔌 Metasearch Integration Guide

> Complete guide for integrating metasearch search functionality into your Rust projects

**What is Metasearch?**  
A blazing-fast, privacy-respecting metasearch engine library with 215+ search engines built in Rust. Add powerful search to your CLI tools, web APIs, desktop apps, or any Rust project.

**Key Stats:**
- 🦀 **215 search engines** (more than SearXNG's 211!)
- ⚡ **~60% working rate** (124/208 tested)
- 🗂️ **9 categories** (General, Images, Videos, News, Music, Files, Science, IT, Map)
- 🚀 **Rust 2024 edition** (requires Rust 1.85.0+)
- 🔒 **Privacy-first** (no tracking, no profiling)

---

## 🚀 Quick Start (3 Steps)

### Step 1: Add Dependencies

```toml
[dependencies]
metasearch-core = { path = "../metasearch/crates/metasearch-core" }
metasearch-engine = { path = "../metasearch/crates/metasearch-engine" }
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
tokio = { version = "1.48", features = ["full"] }
async-trait = "0.1"

# Optional: Fast allocator for better performance
mimalloc = { version = "0.1", default-features = false }
```

### Step 2: Write Code (5 Lines!)

```rust
use metasearch_core::query::SearchQuery;
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = EngineRegistry::with_defaults(Client::new());
    let engine = registry.get("google").unwrap();
    let results = engine.search(&SearchQuery::new("rust")).await?;
    
    for r in results {
        println!("{} - {}", r.title, r.url);
    }
    Ok(())
}
```

### Step 3: Run

```bash
cargo run
```

---

## 🧩 Core Concepts

### 1. Engine Registry

Manages all 215+ search engines:

```rust
let registry = EngineRegistry::with_defaults(Client::new());
println!("Loaded {} engines", registry.count());  // 215

// Get specific engine
let google = registry.get("google").unwrap();

// Get engines by category
let engines = registry.engines_for_category(&SearchCategory::General);

// List all engine names
let names = registry.engine_names();
```

### 2. Search Categories

```rust
use metasearch_core::category::SearchCategory;

SearchCategory::General   // Web search (~80 engines)
SearchCategory::Images    // Image search (~25 engines)
SearchCategory::Videos    // Video search (~20 engines)
SearchCategory::News      // News articles (~15 engines)
SearchCategory::Music     // Music/audio (~15 engines)
SearchCategory::Files     // File downloads (~10 engines)
SearchCategory::Science   // Academic papers (~30 engines)
SearchCategory::It        // IT/programming (~25 engines)
SearchCategory::Map       // Maps/locations (~5 engines)
```

### 3. Search Query

```rust
// Simple
let query = SearchQuery::new("rust programming");

// Advanced
let mut query = SearchQuery::new("rust");
query.page = 2;                          // Pagination
query.language = Some("en".to_string()); // Language filter
query.safe_search = true;                // Filter adult content
```

### 4. Search Results

```rust
pub struct SearchResult {
    pub title: String,              // Result title
    pub url: String,                // Result URL
    pub snippet: String,            // Description
    pub engine: String,             // Engine name
    pub category: String,           // Category
    pub engine_rank: u32,           // Position (1-based)
    pub score: f64,                 // Relevance score
    pub thumbnail: Option<String>,  // Image URL
    pub published_date: Option<String>, // Publication date
}
```

---

## 🔧 Basic Usage

### Single Engine Search

```rust
let client = Client::new();
let google = Google::new(client);
let results = google.search(&SearchQuery::new("rust")).await?;

for result in results {
    println!("{} - {}", result.title, result.url);
}
```

### Multi-Engine Search

```rust
let registry = EngineRegistry::with_defaults(Client::new());
let engines = registry.engines_for_category(&SearchCategory::General);

for engine in engines.iter().take(5) {
    match engine.search(&query).await {
        Ok(results) => println!("✓ {}: {} results", engine.metadata().display_name, results.len()),
        Err(e) => println!("✗ {}: {}", engine.metadata().display_name, e),
    }
}
```

### Parallel Search (10x Faster!)

```rust
use futures::future::join_all;

let tasks: Vec<_> = engines.iter()
    .take(10)
    .map(|engine| {
        let query = query.clone();
        let engine = engine.clone();
        tokio::spawn(async move { engine.search(&query).await })
    })
    .collect();

let results = join_all(tasks).await;
```

---

## 🎯 Advanced Usage

### Result Aggregation & Deduplication

```rust
use metasearch_core::ranking::ResultAggregator;
use dashmap::DashMap;

// Set engine weights (higher = more trusted)
let weights = DashMap::new();
weights.insert("google".to_string(), 1.5);
weights.insert("duckduckgo".to_string(), 1.2);
weights.insert("brave".to_string(), 1.0);

let aggregator = ResultAggregator::new(weights);

// Collect results from multiple engines
let mut all_results = Vec::new();
for engine in engines.iter().take(5) {
    if let Ok(results) = engine.search(&query).await {
        all_results.extend(results);
    }
}

// Aggregate and deduplicate (returns top 50)
let final_results = aggregator.aggregate(all_results, 50);
```

### Autocomplete

```rust
let google = Google::new(client);
let suggestions = google.autocomplete("rust").await?;
// Returns: ["rust programming", "rust game", "rust tutorial", ...]
```

### Category-Specific Search

```rust
// Search images
let engines = registry.engines_for_category(&SearchCategory::Images);

// Search videos
let engines = registry.engines_for_category(&SearchCategory::Videos);

// Search news
let engines = registry.engines_for_category(&SearchCategory::News);
```

### Custom HTTP Client

```rust
use std::time::Duration;

let client = Client::builder()
    .timeout(Duration::from_secs(15))
    .connect_timeout(Duration::from_secs(5))
    .user_agent("MyApp/1.0")
    .gzip(true)
    .brotli(true)
    .pool_max_idle_per_host(10)
    .http2_prior_knowledge()
    .build()?;

let registry = EngineRegistry::with_defaults(client);
```

### Error Handling

```rust
use metasearch_core::error::MetasearchError;

match engine.search(&query).await {
    Ok(results) => println!("✓ {} results", results.len()),
    Err(MetasearchError::HttpError(e)) => eprintln!("HTTP error: {}", e),
    Err(MetasearchError::ParseError(e)) => eprintln!("Parse error: {}", e),
    Err(MetasearchError::Timeout) => eprintln!("Timeout"),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## 🛠️ Custom Engine Development

```rust
use async_trait::async_trait;
use metasearch_core::{
    engine::{EngineMetadata, SearchEngine},
    category::SearchCategory,
    query::SearchQuery,
    result::SearchResult,
    error::Result,
};
use reqwest::Client;
use smallvec::smallvec;

pub struct MyEngine {
    metadata: EngineMetadata,
    client: Client,
}

impl MyEngine {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "my_engine".to_string().into(),
                display_name: "My Engine".to_string().into(),
                homepage: "https://example.com".to_string().into(),
                categories: smallvec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for MyEngine {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!("https://api.example.com/search?q={}", 
            urlencoding::encode(&query.query));
        
        let response = self.client.get(&url).send().await
            .map_err(|e| metasearch_core::error::MetasearchError::HttpError(e.to_string()))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| metasearch_core::error::MetasearchError::ParseError(e.to_string()))?;
        
        let mut results = Vec::new();
        if let Some(items) = json["results"].as_array() {
            for (i, item) in items.iter().enumerate() {
                let title = item["title"].as_str().unwrap_or("").to_string();
                let url = item["url"].as_str().unwrap_or("").to_string();
                let snippet = item["description"].as_str().unwrap_or("").to_string();
                
                let mut result = SearchResult::new(&title, &url, &snippet, "my_engine");
                result.engine_rank = (i + 1) as u32;
                results.push(result);
            }
        }
        
        Ok(results)
    }
}

// Register your engine
use std::sync::Arc;
let mut registry = EngineRegistry::new();
registry.register(Arc::new(MyEngine::new(client)));
```

---

## 💡 Real-World Examples

### Example 1: CLI Search Tool

```rust
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = EngineRegistry::with_defaults(Client::new());
    
    loop {
        print!("Search: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query_text = input.trim();
        
        if query_text.is_empty() || query_text == "quit" {
            break;
        }
        
        let query = SearchQuery::new(query_text);
        let engines = registry.engines_for_category(&SearchCategory::General);
        
        if let Some(engine) = engines.first() {
            match engine.search(&query).await {
                Ok(results) => {
                    for (i, r) in results.iter().take(10).enumerate() {
                        println!("{}. {}\n   {}\n", i + 1, r.title, r.url);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
    Ok(())
}
```

### Example 2: Web API Server

```rust
use axum::{extract::{Query, State}, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    registry: Arc<EngineRegistry>,
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    #[serde(default)]
    engine: Option<String>,
}

#[derive(Serialize)]
struct SearchResponse {
    query: String,
    results: Vec<SearchResult>,
}

async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<SearchResponse> {
    let query = SearchQuery::new(&params.q);
    
    let engine = if let Some(name) = params.engine {
        state.registry.get(&name)
    } else {
        state.registry.engines_for_category(&SearchCategory::General).first().cloned()
    };
    
    let results = if let Some(engine) = engine {
        engine.search(&query).await.unwrap_or_default()
    } else {
        Vec::new()
    };
    
    Json(SearchResponse {
        query: params.q,
        results,
    })
}

#[tokio::main]
async fn main() {
    let registry = Arc::new(EngineRegistry::with_defaults(Client::new()));
    let app = Router::new()
        .route("/search", get(search_handler))
        .with_state(AppState { registry });
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Example 3: Multi-Engine Comparison

```rust
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = EngineRegistry::with_defaults(Client::new());
    let query = SearchQuery::new("rust programming");
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    let mut stats: HashMap<String, (usize, u128)> = HashMap::new();
    
    for engine in engines.iter().take(10) {
        let start = std::time::Instant::now();
        match engine.search(&query).await {
            Ok(results) => {
                let duration = start.elapsed().as_millis();
                stats.insert(engine.metadata().display_name.to_string(), (results.len(), duration));
                println!("✓ {:20} {} results in {}ms", engine.metadata().display_name, results.len(), duration);
            }
            Err(e) => println!("✗ {:20} Error: {}", engine.metadata().display_name, e),
        }
    }
    
    Ok(())
}
```

---

## ⚡ Performance Optimization

### 1. Reuse HTTP Client (Critical!)

```rust
// ✅ DO THIS
let client = Client::new();
let registry = EngineRegistry::with_defaults(client);

// ❌ NOT THIS
for _ in 0..10 {
    let client = Client::new();  // Slow!
}
```

### 2. Use Parallel Search

```rust
// Sequential: ~5 seconds for 5 engines
for engine in engines {
    engine.search(&query).await?;
}

// Parallel: ~1 second for 5 engines (10x faster!)
let tasks: Vec<_> = engines.iter()
    .map(|e| tokio::spawn(e.search(&query)))
    .collect();
join_all(tasks).await;
```

### 3. Configure Timeouts

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(10))
    .connect_timeout(Duration::from_secs(3))
    .build()?;
```

### 4. Use Fast Allocator

```toml
[dependencies]
mimalloc = { version = "0.1", default-features = false }
```

```rust
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```

### 5. Cache Results

```rust
use moka::future::Cache;

let cache: Cache<String, Vec<SearchResult>> = Cache::builder()
    .max_capacity(1000)
    .time_to_live(Duration::from_secs(300))
    .build();

if let Some(results) = cache.get(&query.query).await {
    return Ok(results);
}

let results = engine.search(&query).await?;
cache.insert(query.query.clone(), results.clone()).await;
```

### Performance Comparison

| Method | Time (10 engines) | Speedup |
|--------|-------------------|---------|
| Sequential | ~10 seconds | 1x |
| Parallel | ~1 second | 10x |
| With caching | ~0.1 seconds | 100x |

---

## 🐛 Troubleshooting

### Issue: "edition 2024 not found"

**Solution:** Update Rust
```bash
rustup update
rustc --version  # Should be 1.85.0+
```

### Issue: No Results Returned

**Causes:**
- Bot protection (Cloudflare, CAPTCHA)
- Network timeout
- Engine temporarily down

**Debug:**
```rust
match engine.search(&query).await {
    Err(MetasearchError::HttpError(e)) => println!("HTTP: {}", e),
    Err(MetasearchError::ParseError(e)) => println!("Parse: {}", e),
    Err(MetasearchError::Timeout) => println!("Timeout"),
    _ => {}
}
```

### Issue: Slow Performance

**Solutions:**
- Use parallel search
- Reduce timeout values
- Limit number of engines
- Enable connection pooling
- Use mimalloc allocator

---

## 📊 Engine Statistics

**As of March 2026:**

| Metric | Value |
|--------|-------|
| Total Engines | 215 |
| Working Rate | ~60% (124/208) |
| Categories | 9 |
| Avg Response Time | 0.04s |

**Top Performers:**
1. voidlinux - 309 results
2. www1x - 216 results
3. repology - 200 results
4. lib_rs - 150 results
5. mwmbl - 124 results

---

## 🎯 Best Practices

1. ✅ Reuse HTTP client
2. ✅ Use parallel search for multiple engines
3. ✅ Handle errors gracefully
4. ✅ Configure appropriate timeouts
5. ✅ Cache results when possible
6. ✅ Use result aggregation to deduplicate
7. ✅ Monitor engine performance
8. ✅ Implement rate limiting for production

---

## 📚 Additional Resources

- [README.md](README.md) - Project overview
- [QUICK_START.md](QUICK_START.md) - Server setup
- [ENGINES.md](ENGINES.md) - Complete engine list
- [BRUTAL_TRUTH_REPORT.md](BRUTAL_TRUTH_REPORT.md) - Test results
- GitHub: https://github.com/najmus-sakib-hossain/metasearch

---

## 📝 License

AGPL-3.0 - Free as in freedom

---

## ✅ Summary

You now know how to:
- ✅ Add metasearch to any Rust project (3 steps)
- ✅ Search with 215+ engines
- ✅ Use parallel search for 10x speed
- ✅ Aggregate and deduplicate results
- ✅ Build CLI tools, web APIs, desktop apps
- ✅ Optimize performance
- ✅ Handle errors properly
- ✅ Create custom engines

**Ready to build? Start with the 5-line example at the top!**

---

*Last updated: March 2026 | Metasearch v0.1.0*
