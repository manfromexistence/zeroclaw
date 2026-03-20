# 🔍 Test Metasearch with Mistral AI

## Quick Test Command

### Windows (PowerShell)
```powershell
cargo run --release -- agent --provider mistral --message "Search the web for rust programming language 2026. Use the web search tool to find current information about Rust."
```

### Linux/Mac (Bash)
```bash
cargo run --release -- agent --provider mistral --message "Search the web for rust programming language 2026. Use the web search tool to find current information about Rust."
```

## Or Use the Test Scripts

### Windows
```powershell
.\test_metasearch_with_mistral.ps1
```

### Linux/Mac
```bash
chmod +x test_metasearch_with_mistral.sh
./test_metasearch_with_mistral.sh
```

## What Will Happen

1. ZeroClaw builds (first time may take 5-10 minutes)
2. Mistral AI agent starts
3. Agent receives the search query
4. Agent calls the web_search_tool
5. Metasearch queries 5 engines in parallel:
   - Google
   - DuckDuckGo
   - Brave
   - Bing
   - Yahoo (or others)
6. Results are aggregated and deduplicated
7. Agent receives formatted search results
8. Agent responds with the information

## Expected Output

You should see something like:

```
🤖 Mistral AI Agent

User: Search the web for rust programming language 2026...

Agent: I'll search for that information.

[Tool Call: web_search_tool]
Query: rust programming language 2026

[Tool Result]
Search results for: rust programming language 2026 (via Metasearch - 215+ engines)
1. The Rust Programming Language
   https://www.rust-lang.org/
   A language empowering everyone to build reliable and efficient software.
2. Rust (programming language) - Wikipedia
   https://en.wikipedia.org/wiki/Rust_(programming_language)
   Rust is a multi-paradigm, general-purpose programming language...
3. Rust 2026 Edition Announcement
   https://blog.rust-lang.org/...
   ...

Agent: Based on the search results, Rust is a modern programming language...
```

## Troubleshooting

### If build fails
```bash
cargo check --lib
```

### If Mistral AI not configured
Check your `config.toml` for Mistral API key:
```toml
[providers.mistral]
api_key = "your-api-key-here"
```

### If web search fails
Check that metasearch is enabled in config:
```toml
[tools.web_search]
provider = "metasearch"  # or leave default
```

## Alternative: Interactive Mode

You can also test in interactive mode:

```bash
cargo run --release -- agent --provider mistral
```

Then type:
```
Search the web for rust programming language 2026
```

---

**Ready to test!** Just run the command above. 🚀
