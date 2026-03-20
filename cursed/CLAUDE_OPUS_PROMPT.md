# Implementation Prompt for Claude Opus 4.6

**Project:** ZeroClaw OpenAPI Integration System  
**Goal:** Implement 3,500-4,000 API integrations in 5 weeks  
**Current Status:** 70% infrastructure complete, 30% needs implementation

---

## Context

You are working on ZeroClaw, a Rust-based AI agent framework. The project already has:
- Complete HTTP request tool (`src/tools/http_request.rs`)
- Full MCP (Model Context Protocol) infrastructure (`src/tools/mcp_*.rs`)
- Robust security and auth system (`src/security/`)
- Dynamic tool registry (`src/tools/mod.rs`, `src/tools/traits.rs`)
- Configuration system (`src/config/schema.rs`)

Your task is to implement an OpenAPI-based integration system that will add 3,500-4,000 API integrations by leveraging existing OpenAPI specifications.

---

## Required Reading

Before starting, read these files in order:

1. **CONNECT.md** - The strategic plan and market analysis
2. **cursed/CONNECT_MD_ANALYSIS.md** - Technical feasibility assessment
3. **src/tools/http_request.rs** - Existing HTTP client
4. **src/tools/mcp_client.rs** - MCP infrastructure
5. **src/tools/traits.rs** - Tool trait definition
6. **src/config/schema.rs** - Configuration system

---

## Implementation Plan: Week 1-5 (5 weeks to launch)

### Week 1: Foundation & OpenAPI Parser (Days 1-7)

**Goal:** Parse and validate 1,500-2,000 OpenAPI specs from APIs.guru

#### Tasks:

1. **Add Dependencies to Cargo.toml**
```toml
[dependencies]
openapiv3 = "2.0"           # OpenAPI 3.x parsing
serde_yaml = "0.9"          # YAML spec support
jsonschema = "0.18"         # Schema validation
reqwest = { version = "0.12", features = ["json", "stream"] }
```

2. **Create `src/tools/openapi/mod.rs`**
   - Module structure for OpenAPI subsystem
   - Re-exports for public API

3. **Create `src/tools/openapi/spec.rs`**
```rust
use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    /// Parsed OpenAPI document
    pub spec: OpenAPI,
    /// Base URL for API requests
    pub base_url: String,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Metadata about this spec
    pub metadata: SpecMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecMetadata {
    pub provider: String,
    pub service: String,
    pub version: String,
    pub tier: SpecTier,
    pub quality_score: u8,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecTier {
    Native,      // Hand-crafted Rust (Progenitor)
    Verified,    // Tested, high-quality OpenAPI
    Community,   // Untested, may have issues
    Experimental // AI-generated, needs review
}

impl OpenApiSpec {
    pub fn from_file(path: &Path) -> Result<Self>;
    pub fn from_url(url: &str) -> Result<Self>;
    pub fn validate(&self) -> Result<ValidationReport>;
}
```

4. **Create `src/tools/openapi/validator.rs`**
```rust
pub struct OpenApiValidator;

impl OpenApiValidator {
    pub fn validate(spec: &OpenAPI) -> Result<ValidationReport>;
    pub fn score_quality(spec: &OpenAPI) -> u8;
    pub fn check_required_fields(spec: &OpenAPI) -> Vec<String>;
    pub fn detect_auth_type(spec: &OpenAPI) -> Vec<AuthType>;
}

pub struct ValidationReport {
    pub is_valid: bool,
    pub quality_score: u8,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub missing_fields: Vec<String>,
    pub auth_types: Vec<AuthType>,
}
```

5. **Create `src/tools/openapi/harvester.rs`**
```rust
pub struct SpecHarvester {
    sources: Vec<Box<dyn SpecSource>>,
}

#[async_trait]
pub trait SpecSource: Send + Sync {
    async fn fetch_specs(&self) -> Result<Vec<OpenApiSpec>>;
    fn name(&self) -> &str;
}

pub struct ApisGuruSource {
    repo_path: PathBuf,
}

impl SpecHarvester {
    pub fn new() -> Self;
    pub fn add_source(&mut self, source: Box<dyn SpecSource>);
    pub async fn harvest_all(&self) -> Result<Vec<OpenApiSpec>>;
    pub async fn deduplicate(&self, specs: Vec<OpenApiSpec>) -> Vec<OpenApiSpec>;
}
```

6. **Clone APIs.guru Repository**
   - Add as git submodule or download to `~/.zeroclaw/openapi-specs/`
   - Implement `ApisGuruSource` to parse all specs
   - Handle YAML and JSON formats
   - Deduplicate by provider+service+version

7. **Create CLI Command: `zeroclaw openapi harvest`**
   - Add to `src/main.rs`
   - Harvest specs from all sources
   - Validate and score each spec
   - Save to `~/.zeroclaw/openapi-specs/registry.json`

**Deliverable:** 1,500-2,000 parsed and validated specs

---

### Week 2: Runtime Executor (Days 8-14)

**Goal:** Make specs executable as tools

#### Tasks:

1. **Create `src/tools/openapi/auth.rs`**
```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn apply_auth(&self, req: &mut reqwest::Request) -> Result<()>;
}

pub struct ApiKeyAuth {
    key: String,
    location: KeyLocation,
    param_name: String,
}

pub struct BearerAuth {
    token: String,
}

pub struct BasicAuth {
    username: String,
    password: String,
}

pub struct OAuth2Auth {
    client_id: String,
    client_secret: String,
    token_url: String,
    scopes: Vec<String>,
    token_cache: Arc<Mutex<Option<TokenResponse>>>,
}

#[derive(Debug, Clone)]
pub enum KeyLocation {
    Header,
    Query,
    Cookie,
}

impl OAuth2Auth {
    pub async fn get_token(&self) -> Result<String>;
    pub async fn refresh_token(&self) -> Result<String>;
}
```

2. **Create `src/tools/openapi/executor.rs`**
```rust
pub struct OpenApiTool {
    spec: Arc<OpenApiSpec>,
    operation_id: String,
    operation: Operation,
    http_client: Arc<HttpRequestTool>,
    auth_provider: Option<Arc<dyn AuthProvider>>,
}

impl OpenApiTool {
    pub fn new(
        spec: Arc<OpenApiSpec>,
        operation_id: String,
        http_client: Arc<HttpRequestTool>,
        auth_provider: Option<Arc<dyn AuthProvider>>,
    ) -> Result<Self>;

    fn build_request(&self, args: &Value) -> Result<reqwest::Request>;
    fn serialize_parameters(&self, args: &Value) -> Result<RequestParams>;
    fn parse_response(&self, response: reqwest::Response) -> Result<Value>;
}

struct RequestParams {
    path_params: HashMap<String, String>,
    query_params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Option<Value>,
}

#[async_trait]
impl Tool for OpenApiTool {
    fn name(&self) -> &str {
        &self.operation_id
    }

    fn description(&self) -> &str {
        self.operation.summary.as_deref().unwrap_or("OpenAPI operation")
    }

    fn parameters_schema(&self) -> Value {
        // Convert OpenAPI parameters to JSON Schema
        self.operation_to_json_schema()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        // 1. Build request from args
        let mut req = self.build_request(&args)?;
        
        // 2. Apply authentication
        if let Some(auth) = &self.auth_provider {
            auth.apply_auth(&mut req).await?;
        }
        
        // 3. Execute via HTTP client
        let response = self.http_client.execute_request(req).await?;
        
        // 4. Parse response
        let result = self.parse_response(response)?;
        
        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }
}
```

3. **Create `src/tools/openapi/registry.rs`**
```rust
pub struct OpenApiRegistry {
    specs: HashMap<String, Arc<OpenApiSpec>>,
    tools: HashMap<String, Arc<OpenApiTool>>,
}

impl OpenApiRegistry {
    pub fn new() -> Self;
    pub fn load_from_disk(&mut self, path: &Path) -> Result<()>;
    pub fn register_spec(&mut self, spec: OpenApiSpec) -> Result<()>;
    pub fn create_tools(&mut self, spec_id: &str) -> Result<Vec<Arc<OpenApiTool>>>;
    pub fn get_tool(&self, name: &str) -> Option<Arc<OpenApiTool>>;
    pub fn list_tools(&self) -> Vec<String>;
    pub fn search_tools(&self, query: &str) -> Vec<String>;
}
```

4. **Integrate with Tool System**
   - Modify `src/tools/mod.rs` to include OpenAPI tools
   - Add `openapi_registry: Option<Arc<OpenApiRegistry>>` to tool initialization
   - Register all OpenAPI tools in `all_tools_with_runtime()`

5. **Add Configuration**
   - Add `[openapi]` section to `src/config/schema.rs`
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenApiConfig {
    pub enabled: bool,
    pub specs_dir: String,
    pub auto_load: bool,
    pub default_tier: SpecTier,
    pub auth: HashMap<String, AuthConfig>,
}
```

6. **Create CLI Commands**
   - `zeroclaw openapi list` - List all loaded specs
   - `zeroclaw openapi tools <spec>` - List tools for a spec
   - `zeroclaw openapi test <tool>` - Test a tool execution

**Deliverable:** 1,500-2,000 executable OpenAPI tools

---

### Week 3: Expansion & Converters (Days 15-21)

**Goal:** Add Postman, AWS, Google specs to reach 2,500-3,500 tools

#### Tasks:

1. **Create `src/tools/openapi/converters/mod.rs`**
   - Module for format converters

2. **Create `src/tools/openapi/converters/postman.rs`**
```rust
pub struct PostmanConverter;

impl PostmanConverter {
    pub async fn convert_collection(url: &str) -> Result<OpenApiSpec>;
    pub async fn convert_file(path: &Path) -> Result<OpenApiSpec>;
}

// Use external tool: postman-to-openapi npm package
// Or: Postman API endpoint
```

3. **Create `src/tools/openapi/converters/aws.rs`**
```rust
pub struct AwsSmithyConverter;

impl AwsSmithyConverter {
    pub async fn convert_service(service: &str) -> Result<OpenApiSpec>;
    pub async fn list_services() -> Result<Vec<String>>;
}

// Convert AWS Smithy models to OpenAPI
// Use aws-sdk-rust service definitions
```

4. **Create `src/tools/openapi/converters/google.rs`**
```rust
pub struct GoogleDiscoveryConverter;

impl GoogleDiscoveryConverter {
    pub async fn convert_api(api: &str) -> Result<OpenApiSpec>;
    pub async fn list_apis() -> Result<Vec<String>>;
}

// Convert Google Discovery format to OpenAPI
// Use Google's discovery API
```

5. **Implement Conversion Pipeline**
   - Add converters to `SpecHarvester`
   - Run conversions in parallel
   - Validate converted specs
   - Assign quality scores

6. **Create CLI Commands**
   - `zeroclaw openapi convert postman <url>`
   - `zeroclaw openapi convert aws <service>`
   - `zeroclaw openapi convert google <api>`

**Deliverable:** 2,500-3,500 tools (including conversions)

---

### Week 4: Polish & Native Tools (Days 22-28)

**Goal:** Add OAuth2, hand-craft top 10 tools, create MCP server

#### Tasks:

1. **Implement OAuth2 Flow**
   - Complete `OAuth2Auth` in `auth.rs`
   - Add token refresh logic
   - Integrate with secrets storage
   - Add OAuth2 configuration UI/CLI

2. **Hand-Craft Top 10 Native Tools**
   - Use Progenitor for high-quality specs
   - Create dedicated modules in `src/tools/`
   - Examples: GitHub, Stripe, Slack, Google Drive, etc.
```rust
// src/tools/github_api.rs
pub struct GitHubTool {
    client: github_client::Client, // Generated by Progenitor
}
```

3. **Create MCP Server for OpenAPI Tools**
```rust
// src/tools/openapi/mcp_server.rs
pub struct OpenApiMcpServer {
    registry: Arc<OpenApiRegistry>,
}

impl OpenApiMcpServer {
    pub async fn start(&self, port: u16) -> Result<()>;
    pub async fn handle_tools_list(&self) -> Result<McpToolsListResult>;
    pub async fn handle_tool_call(&self, name: &str, args: Value) -> Result<Value>;
}
```

4. **Add CLI Commands**
   - `zeroclaw openapi mcp start` - Start MCP server
   - `zeroclaw openapi mcp test` - Test MCP server

5. **Documentation**
   - Create `docs/OPENAPI.md` with usage guide
   - Add examples for common APIs
   - Document auth configuration
   - Add troubleshooting guide

**Deliverable:** 3,000-4,000 tools + MCP server + docs

---

### Week 5: Launch Preparation (Days 29-35)

**Goal:** Community pipeline, polish, launch

#### Tasks:

1. **Create Community Submission Pipeline**
   - GitHub Actions workflow for PR validation
   - Automated spec testing
   - Quality scoring
   - Tier assignment

2. **Create GitHub Repository Structure**
```
zeroclaw-openapi-specs/
├── specs/
│   ├── verified/
│   ├── community/
│   └── experimental/
├── .github/
│   └── workflows/
│       └── validate-spec.yml
├── CONTRIBUTING.md
└── README.md
```

3. **Polish CLI Experience**
   - Add progress bars for harvesting
   - Add colored output
   - Add interactive tool selection
   - Add `zeroclaw openapi search <query>`

4. **Create Marketing Materials**
   - Blog post draft
   - HackerNews post
   - Twitter thread
   - Demo video script

5. **Final Testing**
   - Test top 50 most popular APIs
   - Fix critical bugs
   - Performance optimization
   - Security audit

6. **Launch Checklist**
   - [ ] All tests passing
   - [ ] Documentation complete
   - [ ] MCP server working
   - [ ] CLI polished
   - [ ] GitHub repo ready
   - [ ] Marketing materials ready

**Deliverable:** Public launch with 3,500-4,000 tools

---

## Technical Requirements

### Code Quality Standards

1. **Error Handling**
   - Use `anyhow::Result` for all fallible operations
   - Provide context with `.context()` or `.with_context()`
   - Never use `.unwrap()` in production code

2. **Async/Await**
   - Use `tokio` runtime
   - Use `async_trait` for trait methods
   - Handle timeouts with `tokio::time::timeout`

3. **Testing**
   - Unit tests for all core functions
   - Integration tests for tool execution
   - Mock HTTP responses for tests
   - Test error cases

4. **Documentation**
   - Doc comments for all public items
   - Examples in doc comments
   - Module-level documentation

5. **Performance**
   - Use `Arc` for shared data
   - Use `RwLock` for concurrent access
   - Avoid cloning large structures
   - Stream large responses

### Security Requirements

1. **Input Validation**
   - Validate all user inputs
   - Sanitize URLs
   - Check domain allowlists
   - Prevent SSRF attacks

2. **Authentication**
   - Store secrets encrypted
   - Never log sensitive data
   - Use secure token storage
   - Implement token refresh

3. **Rate Limiting**
   - Respect API rate limits
   - Implement backoff strategies
   - Cache responses when appropriate

### Integration Points

1. **Existing HTTP Client**
   - Reuse `HttpRequestTool` for all requests
   - Leverage existing security checks
   - Use existing proxy configuration

2. **Existing MCP Infrastructure**
   - Follow `McpServer` patterns
   - Use `McpRegistry` for registration
   - Implement `McpTransportConn` if needed

3. **Existing Configuration**
   - Add `[openapi]` section to config
   - Follow existing config patterns
   - Support environment variables

4. **Existing Tool System**
   - Implement `Tool` trait
   - Register in `all_tools_with_runtime()`
   - Support tool search

---

## Success Criteria

### Week 1
- [ ] 1,500-2,000 specs parsed and validated
- [ ] Quality scoring working
- [ ] Deduplication working
- [ ] CLI commands functional

### Week 2
- [ ] All specs executable as tools
- [ ] Parameter serialization working
- [ ] Response parsing working
- [ ] Basic auth (API key, Bearer) working

### Week 3
- [ ] Postman conversion working
- [ ] AWS conversion working
- [ ] Google conversion working
- [ ] 2,500-3,500 total tools

### Week 4
- [ ] OAuth2 flow working
- [ ] Top 10 native tools complete
- [ ] MCP server functional
- [ ] Documentation complete

### Week 5
- [ ] Community pipeline ready
- [ ] GitHub repo live
- [ ] Marketing materials ready
- [ ] 3,500-4,000 tools ready for launch

---

## Common Pitfalls to Avoid

1. **Don't over-engineer Week 1**
   - Start simple, iterate later
   - Focus on getting specs loaded first
   - Polish can come in Week 4-5

2. **Don't ignore spec quality**
   - Many real-world specs are broken
   - Implement validation early
   - Use quality tiers from Day 1

3. **Don't build custom HTTP client**
   - Reuse existing `HttpRequestTool`
   - Leverage existing security
   - Don't reinvent the wheel

4. **Don't skip testing**
   - Test with real APIs early
   - Mock responses for unit tests
   - Test error cases

5. **Don't forget auth**
   - Many APIs require auth
   - Implement API key/Bearer in Week 2
   - OAuth2 can wait until Week 4

6. **Don't claim 10,000 integrations**
   - Be honest: 3,500-4,000 in Week 5
   - Quality over quantity
   - Community will add more later

---

## Questions to Ask Before Starting

1. **Clarify scope:** Should I implement all 5 weeks or start with Week 1?
2. **Clarify priorities:** Which APIs are most important to support first?
3. **Clarify auth:** Should OAuth2 be in Week 2 or Week 4?
4. **Clarify testing:** What level of test coverage is expected?
5. **Clarify deployment:** How should specs be distributed (git submodule, download, embedded)?

---

## Example Usage (Target UX)

```bash
# Harvest specs from all sources
zeroclaw openapi harvest

# List available specs
zeroclaw openapi list

# Show tools for a specific spec
zeroclaw openapi tools github

# Test a tool
zeroclaw openapi test github__create_issue \
  --args '{"owner":"user","repo":"repo","title":"Test"}'

# Add a custom spec
zeroclaw openapi add https://api.example.com/openapi.json

# Start MCP server
zeroclaw openapi mcp start --port 3000

# Search for tools
zeroclaw openapi search "create issue"
```

```rust
// Programmatic usage
use zeroclaw::tools::openapi::{OpenApiRegistry, OpenApiTool};

let mut registry = OpenApiRegistry::new();
registry.load_from_disk("~/.zeroclaw/openapi-specs")?;

let tool = registry.get_tool("github__create_issue")?;
let result = tool.execute(json!({
    "owner": "user",
    "repo": "repo",
    "title": "Test issue"
})).await?;

println!("{}", result.output);
```

---

## Final Notes

- **Start with Week 1** - Get the foundation right
- **Test early and often** - Use real APIs from Day 1
- **Keep it simple** - Don't over-engineer
- **Focus on quality** - 3,500 good tools > 10,000 broken tools
- **Document as you go** - Future you will thank you
- **Ask questions** - Clarify before implementing

Good luck! 🚀

---

## Appendix: File Structure

```
src/tools/openapi/
├── mod.rs              # Module exports
├── spec.rs             # OpenApiSpec struct
├── validator.rs        # Spec validation
├── harvester.rs        # Spec collection
├── executor.rs         # OpenApiTool implementation
├── auth.rs             # Auth providers
├── registry.rs         # Tool registry
├── mcp_server.rs       # MCP server
└── converters/
    ├── mod.rs
    ├── postman.rs
    ├── aws.rs
    └── google.rs
```

```
~/.zeroclaw/openapi-specs/
├── registry.json       # Metadata for all specs
├── specs/
│   ├── github.yaml
│   ├── stripe.yaml
│   └── ...
└── cache/
    └── tokens/         # OAuth2 token cache
```
