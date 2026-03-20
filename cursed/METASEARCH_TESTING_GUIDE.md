# 🔍 Metasearch Testing Guide

## Overview

Metasearch has been successfully integrated into Agent with 215+ search engines. This guide shows you how to test and verify the integration.

## ✅ Integration Status

- **215 search engines** integrated into `src/metasearch/`
- **Web search tool** configured to use metasearch by default
- **All dependencies** added to Cargo.toml
- **Test suite** created
- **Build status:** ✅ PASSING

## 🧪 Testing Methods

### Method 1: Run Unit Tests

```bash
# Test engine registry (verifies all 215+ engines load)
cargo test test_metasearch_engine_registry -- --nocapture

# Test categories (verifies engines are categorized correctly)
cargo test test_metasearch_categories -- --nocapture

# Test result aggregation (verifies deduplication works)
cargo test test_metasearch_result_aggregation -- --nocapture

# Run all metasearch tests
cargo test test_metasearch -- --nocapture
```

### Method 2: Run Example Program

```bash
# Run the standalone test example
cargo run --example test_metasearch
```

Expected output:
```
🔍 Testing Metasearch Integration...

✅ Loaded 215 search engines
  ✓ google engine available
  ✓ duckduckgo engine available
  ✓ brave engine available
  ✓ bing engine available
  ✓ yahoo engine available

📊 Engines by category:
  - General: 80 engines
  - Images: 25 engines
  - Videos: 20 engines

✅ Metasearch integration test PASSED!
```

### Method 3: Live Search Test (Requires Network)

```bash
# This actually queries real search engines
cargo test test_metasearch_live_search -- --ignored --nocapture
```

This will:
1. Query 5 search engines in parallel
2. Aggregate and deduplicate results
3. Display formatted search results
4. Show search time

### Method 4: Test with Agent Agent

If you have a Mistral AI agent configured:

```bash
# Start Agent
cargo run

# In the agent prompt, ask:
"Search the web for rust programming language 2026"
```

The agent will use the metasearch tool automatically, querying multiple engines and returning aggregated results.

## 📊 What Gets Tested

### Engine Registry Test
- Verifies all 215+ engines are loaded
- Checks specific engines (Google, DuckDuckGo, Brave, Bing, Yahoo)
- Confirms no engines are missing

### Category Test
- Verifies engines are properly categorized
- Tests: General, Images, Videos, News, Music, Files, Science, IT, Maps
- Ensures each category has engines

### Parallel Search Test
- Tests searching 3 engines simultaneously
- Verifies parallel execution works
- Checks that at least one engine returns results

### Result Aggregation Test
- Tests deduplication (same URL from different engines)
- Verifies weighted ranking (Google 1.5x, DuckDuckGo 1.2x)
- Checks result merging

### Live Search Test
- Actually queries real search engines
- Tests network connectivity
- Verifies end-to-end functionality
- Measures search performance

## 🔧 Troubleshooting

### Build Takes Long Time
First build may take 5-10 minutes as it compiles 215 engine implementations. Subsequent builds are much faster.

```bash
# Use release-fast profile for faster compilation
cargo build --profile release-fast
```

### Network Timeout
Some engines may timeout or be blocked by bot detection. This is expected. The tool will:
1. Try 5 engines in parallel
2. Use results from engines that succeed
3. Fall back to legacy providers if all fail

### No Results Returned
If metasearch returns no results:
1. Check internet connectivity
2. Try the legacy fallback: set `provider = "duckduckgo"` in config
3. Run with `--nocapture` to see detailed logs

## 📈 Performance Expectations

- **Engine loading:** < 1 second (215 engines)
- **Parallel search:** 1-3 seconds (5 engines)
- **Result aggregation:** < 100ms
- **Total search time:** 1-5 seconds

## 🎯 Success Criteria

✅ All tests pass
✅ At least 200 engines load successfully
✅ Parallel search returns results from multiple engines
✅ Results are deduplicated correctly
✅ Search completes in < 5 seconds

## 📝 Example Test Output

```
running 1 test
🔍 Testing Metasearch Integration...

✅ Loaded 215 search engines
  ✓ google engine available
  ✓ duckduckgo engine available
  ✓ brave engine available
  ✓ bing engine available
  ✓ yahoo engine available

📊 Engines by category:
  - General: 80 engines
  - Images: 25 engines
  - Videos: 20 engines

✅ Metasearch integration test PASSED!
test test_metasearch_engine_registry ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 🚀 Next Steps After Testing

1. **Configure engine weights** in `src/tools/web_search_tool.rs`
2. **Add more engines** by creating new files in `src/metasearch/engines/`
3. **Customize categories** in `src/metasearch/category.rs`
4. **Tune performance** by adjusting parallel engine count
5. **Add caching** for frequently searched queries

## 📚 Additional Resources

- **Integration docs:** `cursed/METASEARCH_INTEGRATION_COMPLETE.md`
- **Engine implementations:** `src/metasearch/engines/`
- **Core types:** `src/metasearch/`
- **Web search tool:** `src/tools/web_search_tool.rs`

## 🐛 Reporting Issues

If you encounter issues:
1. Run with `--nocapture` to see detailed logs
2. Check `cargo check --lib` for compilation errors
3. Verify dependencies in `Cargo.toml`
4. Check network connectivity for live tests

---

**Status:** ✅ Ready for Testing
**Last Updated:** March 20, 2026
