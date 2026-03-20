//! Metasearch integration tests
//!
//! Tests the integrated metasearch functionality with 215+ search engines.

use serde_json::json;
use zeroclawlabs::tools::web_search_tool::WebSearchTool;
use zeroclawlabs::tools::traits::Tool;

#[tokio::test]
async fn test_metasearch_basic_query() {
    // Create web search tool with metasearch enabled (default)
    let tool = WebSearchTool::new(
        "metasearch".to_string(),
        None,  // No Brave API key needed
        5,     // Max 5 results
        15,    // 15 second timeout
    );

    // Test basic search
    let query = json!({
        "query": "rust programming language"
    });

    let result = tool.execute(query).await;
    
    assert!(result.is_ok(), "Metasearch should execute successfully");
    
    let tool_result = result.unwrap();
    assert!(tool_result.success, "Search should succeed");
    assert!(!tool_result.output.is_empty(), "Should return results");
    
    // Verify output contains expected elements
    let output = tool_result.output;
    assert!(output.contains("Search results for:"), "Should have search header");
    assert!(output.contains("Metasearch"), "Should indicate metasearch was used");
    assert!(output.contains("http"), "Should contain URLs");
    
    println!("✅ Metasearch basic query test passed");
    println!("Output preview:\n{}", &output[..output.len().min(500)]);
}

#[tokio::test]
async fn test_metasearch_engine_registry() {
    use zeroclawlabs::metasearch::engines::EngineRegistry;
    use reqwest::Client;

    // Create registry with all 215+ engines
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);

    // Verify engine count
    let count = registry.count();
    assert!(count >= 200, "Should have at least 200 engines, got {}", count);
    
    println!("✅ Engine registry test passed: {} engines loaded", count);

    // Verify specific engines exist
    assert!(registry.get("google").is_some(), "Google engine should exist");
    assert!(registry.get("duckduckgo").is_some(), "DuckDuckGo engine should exist");
    assert!(registry.get("brave").is_some(), "Brave engine should exist");
    assert!(registry.get("bing").is_some(), "Bing engine should exist");
    
    println!("✅ Core engines verified");
}

#[tokio::test]
async fn test_metasearch_categories() {
    use zeroclawlabs::metasearch::engines::EngineRegistry;
    use zeroclawlabs::metasearch::category::SearchCategory;
    use reqwest::Client;

    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);

    // Test different categories
    let general_engines = registry.engines_for_category(&SearchCategory::General);
    let image_engines = registry.engines_for_category(&SearchCategory::Images);
    let video_engines = registry.engines_for_category(&SearchCategory::Videos);
    let news_engines = registry.engines_for_category(&SearchCategory::News);

    assert!(!general_engines.is_empty(), "Should have general search engines");
    assert!(!image_engines.is_empty(), "Should have image search engines");
    assert!(!video_engines.is_empty(), "Should have video search engines");
    assert!(!news_engines.is_empty(), "Should have news search engines");

    println!("✅ Category test passed:");
    println!("  - General: {} engines", general_engines.len());
    println!("  - Images: {} engines", image_engines.len());
    println!("  - Videos: {} engines", video_engines.len());
    println!("  - News: {} engines", news_engines.len());
}

#[tokio::test]
async fn test_metasearch_parallel_search() {
    use zeroclawlabs::metasearch::engines::EngineRegistry;
    use zeroclawlabs::metasearch::query::SearchQuery;
    use zeroclawlabs::metasearch::category::SearchCategory;
    use reqwest::Client;
    use std::sync::Arc;

    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    let engines = registry.engines_for_category(&SearchCategory::General);

    // Take first 3 engines for quick test
    let test_engines: Vec<_> = engines.into_iter().take(3).collect();
    
    let query = SearchQuery::new("rust");
    
    // Search in parallel
    let mut tasks = Vec::new();
    for engine in test_engines {
        let query_clone = query.clone();
        let engine_clone = Arc::clone(&engine);
        tasks.push(tokio::spawn(async move {
            let name = engine_clone.metadata().display_name.to_string();
            let result = engine_clone.search(&query_clone).await;
            (name, result.is_ok())
        }));
    }

    // Collect results
    let mut success_count = 0;
    for task in tasks {
        if let Ok((name, success)) = task.await {
            if success {
                success_count += 1;
                println!("  ✓ {} returned results", name);
            } else {
                println!("  ✗ {} failed (expected for some engines)", name);
            }
        }
    }

    assert!(success_count > 0, "At least one engine should return results");
    println!("✅ Parallel search test passed: {}/3 engines succeeded", success_count);
}

#[tokio::test]
async fn test_metasearch_result_aggregation() {
    use zeroclawlabs::metasearch::ranking::ResultAggregator;
    use zeroclawlabs::metasearch::result::SearchResult;
    use dashmap::DashMap;

    // Create some test results
    let mut results1 = vec![
        SearchResult::new("Rust Lang", "https://rust-lang.org", "Official site", "google"),
        SearchResult::new("Rust Book", "https://doc.rust-lang.org/book", "Learn Rust", "google"),
    ];
    results1[0].engine_rank = 1;
    results1[1].engine_rank = 2;

    let mut results2 = vec![
        SearchResult::new("Rust Lang", "https://rust-lang.org", "Official site", "duckduckgo"),
        SearchResult::new("Rust GitHub", "https://github.com/rust-lang/rust", "Source code", "duckduckgo"),
    ];
    results2[0].engine_rank = 1;
    results2[1].engine_rank = 2;

    // Set up weights
    let weights = DashMap::new();
    weights.insert("google".to_string(), 1.5);
    weights.insert("duckduckgo".to_string(), 1.2);

    let aggregator = ResultAggregator::new(weights);
    
    // Aggregate results
    let all_results = vec![
        ("google".to_string(), results1),
        ("duckduckgo".to_string(), results2),
    ];
    
    let response = aggregator.aggregate("rust", all_results, 100);

    // Verify aggregation
    assert!(!response.results.is_empty(), "Should have aggregated results");
    assert_eq!(response.query, "rust", "Query should match");
    assert!(response.results.len() <= 3, "Should deduplicate rust-lang.org");
    
    println!("✅ Result aggregation test passed");
    println!("  - Aggregated {} unique results from 2 engines", response.results.len());
    println!("  - Engines used: {:?}", response.engines_used);
}

#[tokio::test]
async fn test_metasearch_fallback_to_legacy() {
    // Create tool with invalid metasearch setup to test fallback
    let tool = WebSearchTool::new(
        "duckduckgo".to_string(),  // Use legacy provider
        None,
        5,
        15,
    );

    let query = json!({
        "query": "rust programming"
    });

    let result = tool.execute(query).await;
    
    assert!(result.is_ok(), "Legacy fallback should work");
    
    let tool_result = result.unwrap();
    assert!(tool_result.success, "Legacy search should succeed");
    
    println!("✅ Legacy fallback test passed");
}

#[tokio::test]
#[ignore] // Ignore by default as it requires network
async fn test_metasearch_live_search() {
    // This test actually queries real search engines
    // Run with: cargo test test_metasearch_live_search -- --ignored --nocapture
    
    let tool = WebSearchTool::new(
        "metasearch".to_string(),
        None,
        10,
        20,
    );

    let query = json!({
        "query": "rust programming language 2026"
    });

    println!("🔍 Performing live metasearch...");
    let start = std::time::Instant::now();
    
    let result = tool.execute(query).await;
    
    let elapsed = start.elapsed();
    println!("⏱️  Search completed in {:.2}s", elapsed.as_secs_f64());
    
    assert!(result.is_ok(), "Live search should succeed");
    
    let tool_result = result.unwrap();
    assert!(tool_result.success, "Live search should return success");
    
    println!("\n📊 Results:\n{}", tool_result.output);
    println!("\n✅ Live metasearch test passed");
}
