#!/bin/bash
# Verify metasearch integration

echo "🔍 Verifying Metasearch Integration..."
echo ""

# Check if metasearch module exists
if [ -d "src/metasearch" ]; then
    echo "✅ Metasearch module found at src/metasearch/"
else
    echo "❌ Metasearch module not found"
    exit 1
fi

# Count engine files
ENGINE_COUNT=$(find src/metasearch/engines -name "*.rs" -type f ! -name "mod.rs" ! -name "registry.rs" | wc -l)
echo "✅ Found $ENGINE_COUNT engine implementations"

# Check core modules
CORE_MODULES=("category.rs" "config.rs" "engine.rs" "error.rs" "query.rs" "ranking.rs" "result.rs" "mod.rs")
echo ""
echo "📦 Core modules:"
for module in "${CORE_MODULES[@]}"; do
    if [ -f "src/metasearch/$module" ]; then
        echo "  ✓ $module"
    else
        echo "  ✗ $module (missing)"
    fi
done

# Check if web_search_tool imports metasearch
echo ""
echo "🔗 Integration check:"
if grep -q "use crate::metasearch::" "src/tools/web_search_tool.rs"; then
    echo "  ✓ web_search_tool.rs imports metasearch"
else
    echo "  ✗ web_search_tool.rs doesn't import metasearch"
fi

# Check Cargo.toml dependencies
echo ""
echo "📦 Dependencies:"
DEPS=("scraper" "html-escape" "rayon" "compact_str" "ahash" "dashmap" "smallvec" "url")
for dep in "${DEPS[@]}"; do
    if grep -q "^$dep = " "Cargo.toml"; then
        echo "  ✓ $dep"
    else
        echo "  ✗ $dep (missing)"
    fi
done

echo ""
echo "✅ Metasearch integration verification complete!"
echo ""
echo "To test functionality, run:"
echo "  cargo test test_metasearch_engine_registry -- --nocapture"
echo "  cargo run --example test_metasearch"
