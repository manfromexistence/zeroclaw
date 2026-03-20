# Help Needed - Module Import Issue

**Date:** 2026-03-21

## Problem: Cannot Import metasearch Module in web_search_tool.rs

### Task Description
Attempting to import the `metasearch` module (located at `src/metasearch/`) into `src/tools/web_search_tool.rs` to use the integrated 215+ search engines.

### Current Error
```
error[E0432]: unresolved import `super::metasearch`
  --> src\tools\web_search_tool.rs:10:12
   |
10 | use super::metasearch::{
   |            ^^^^^^^^^^ could not find `metasearch` in `super`
```

### What Was Tried

**Attempt 1: Using `crate::metasearch`**
```rust
use crate::metasearch::{SearchCategory, SearchQuery, ...};
```
- Result: Works for `cargo check --lib` but fails for `cargo check --bin agent`
- Error: "unresolved import `crate::metasearch`" when building binary
- Reason: In binary context, `crate::` refers to the binary crate, not the library

**Attempt 2: Using `agentlabs::metasearch`**
```rust
use agentlabs::metasearch::{SearchCategory, SearchQuery, ...};
```
- Result: Works for binary but fails for library
- Error: "use of unresolved module or unlinked crate `agentlabs`"
- Reason: Within the library, can't reference the library by its external name

**Attempt 3: Using `super::metasearch`**
```rust
use super::metasearch::{SearchCategory, SearchQuery, ...};
```
- Result: Fails for both lib and binary
- Error: "could not find `metasearch` in `super`"
- Reason: `super` from `tools/web_search_tool.rs` refers to the `tools` module, not crate root

**Attempt 4: Using `super::super::metasearch`**
```rust
use super::super::metasearch::{SearchCategory, SearchQuery, ...};
```
- Result: Fails
- Error: Similar unresolved import
- Reason: Module path doesn't resolve correctly

**Attempt 5: Fully qualified paths inline**
```rust
let search_query = crate::metasearch::SearchQuery::new(query);
```
- Result: Same issue as Attempt 1

### Module Structure

```
src/
├── lib.rs                    # Declares: pub mod metasearch; pub mod tools;
├── metasearch/
│   ├── mod.rs               # Re-exports: SearchCategory, SearchQuery, etc.
│   ├── category.rs
│   ├── query.rs
│   ├── ranking.rs
│   └── engines/
│       └── registry.rs
└── tools/
    ├── mod.rs               # Declares: pub mod web_search_tool;
    └── web_search_tool.rs   # NEEDS TO IMPORT metasearch types
```

### Key Facts

1. `metasearch` is declared as `pub mod metasearch;` in `src/lib.rs` (line 51)
2. `tools` is declared as `pub mod tools;` in `src/lib.rs` (line 75)
3. Both modules are siblings at the crate root level
4. `cargo check --lib` succeeds when NOT importing metasearch
5. The metasearch module itself compiles successfully
6. All metasearch types are properly re-exported in `src/metasearch/mod.rs`

### Environment Info

- Language: Rust edition 2024
- Compiler: rustc 1.94.0
- Project: Library + Binary in same crate
- Crate name: `agentlabs`
- Binary name: `agent`

### What Needs to Work

The `web_search_tool.rs` needs to import and use these types:
- `SearchCategory` (enum)
- `SearchQuery` (struct)
- `ResultAggregator` (struct)
- `EngineRegistry` (struct)
- `SearchEngine` (trait)

### Suggested Solutions to Try

1. **Check if there's a circular dependency** between tools and metasearch modules
2. **Verify module visibility** - ensure all parent modules properly declare submodules
3. **Try using `#[path]` attribute** to explicitly specify module location
4. **Consider restructuring** - move web_search_tool to a different location
5. **Check for name conflicts** - ensure no shadowing of module names
6. **Verify Cargo.toml** - ensure no workspace issues affecting module resolution
7. **Try explicit re-export** in tools/mod.rs: `pub use crate::metasearch;`

### Additional Context

This issue arose after successfully integrating:
- Metasearch system (215+ engines) from `metasearch/` folder to `src/metasearch/`
- Token-saving system (RLM) from `token/` folder to `src/token/`
- 42 additional tools from `tools/` folder to `src/tools/`

The metasearch integration itself was successful and compiles. The issue is purely with importing it into web_search_tool.

### Files to Review

- `src/lib.rs` - Module declarations
- `src/tools/mod.rs` - Tools module structure
- `src/tools/web_search_tool.rs` - File with import issue
- `src/metasearch/mod.rs` - Metasearch exports
- `Cargo.toml` - Crate configuration
