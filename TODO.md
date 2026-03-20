
Done

•  Read required context and architecture files:
◦  CONNECT.md
◦  cursed/CONNECT_MD_ANALYSIS.md
◦  src/tools/http_request.rs
◦  src/tools/mcp_client.rs
◦  src/tools/traits.rs
◦  src/config/schema.rs
•  Added OpenAPI dependencies in Cargo.toml:
◦  openapiv3
◦  jsonschema
•  Added Week 1 OpenAPI foundation module:
◦  src/tools/openapi/mod.rs
◦  src/tools/openapi/spec.rs
◦  src/tools/openapi/validator.rs
◦  src/tools/openapi/harvester.rs
•  Implemented core Week 1 structures:
◦  OpenApiSpec, SpecMetadata, SpecTier
◦  AuthConfig, AuthType
◦  OpenApiValidator, ValidationReport
◦  SpecHarvester, SpecSource, ApisGuruSource
•  Implemented parsing/validation basics:
◦  Load OpenAPI from YAML/JSON files
◦  URL loader path (from_url)
◦  Required fields checks
◦  Auth type detection
◦  Quality scoring
◦  dedup key and dedup pipeline
•  Added OpenAPI config to schema:
◦  new OpenApiConfig in src/config/schema.rs
◦  added openapi field to Config
◦  wired defaults into config builders and onboarding config creation
•  Added CLI command path:
◦  new subcommand zeroclaw openapi harvest [--source <path>]
◦  added command handler at src/commands/openapi.rs
◦  writes registry to ~/.zeroclaw/openapi-specs/registry.json (from config path expansion)
•  Wired module exports:
◦  src/tools/mod.rs
◦  src/commands/mod.rs
•  Fixed integration/compile issues after wiring and refactors.
•  Verified with cargo check (successful).
•  Week 2 Runtime Executor (COMPLETE):
◦  Created src/tools/openapi/auth.rs with full auth providers:
▪  NoAuth, ApiKeyAuth, BearerAuth, BasicAuth, OAuth2Auth
▪  OAuth2 with automatic token refresh and caching
▪  create_auth_provider factory function
◦  Created src/tools/openapi/executor.rs:
▪  OpenApiTool implementing Tool trait
▪  Request building from OpenAPI operations
▪  Parameter serialization (path, query, header, body)
▪  Response parsing
▪  JSON Schema generation from OpenAPI parameters
◦  Created src/tools/openapi/registry.rs:
▪  OpenApiRegistry for managing specs and tools
▪  load_from_disk() for loading from registry.json
▪  register_spec() for dynamic spec registration
▪  Tool generation from OpenAPI operations
▪  Tool search and lookup functions
◦  Updated src/tools/openapi/mod.rs with all exports
◦  All modules compile successfully
•  Week 2 Integration (COMPLETE):
◦  Integrated OpenAPI tools into all_tools_with_runtime()
◦  Added CLI commands:
▪  zeroclaw openapi harvest
▪  zeroclaw openapi list
▪  zeroclaw openapi tools <spec>
▪  zeroclaw openapi test <tool> --args '{}'
▪  zeroclaw openapi search <query>
◦  Wired OpenApiRegistry into tool system
◦  All code compiles successfully (0 errors, 0 warnings)

TODO

•  Week 2 Testing (NEXT - HIGH PRIORITY):
◦  Download APIs.guru repository
◦  Run zeroclaw openapi harvest and verify registry creation
◦  Test zeroclaw openapi list
◦  Test tool execution with real API
◦  Fix any runtime issues
•  Week 1 completion gaps:
◦  Add stronger validation/normalization for broken real-world specs
◦  Implement robust metadata extraction for provider/service/version beyond path heuristics
◦  Add tests for spec.rs, validator.rs, harvester.rs
◦  Add APIs.guru fetch/bootstrap workflow (submodule or download flow)
•  Quality/Security:
◦  Reuse HttpRequestTool end-to-end for OpenAPI execution path
◦  Add SSRF/domain checks in all OpenAPI request flows
◦  Add auth secret handling with encrypted storage integration
◦  Add timeout/retry/rate-limit behavior
•  Testing/Validation:
◦  Unit tests for parameter serialization and response parsing
◦  Integration tests for harvest + execution
◦  Run lint/test matrix and fix failures
•  Later phases (Weeks 3–5):
◦  Converters (postman, aws, google)
◦  MCP server for OpenAPI tools
◦  Documentation (docs/OPENAPI.md)
◦  Community validation pipeline and launch polish
