# ZeroClaw OpenAPI Integration - TODO

## COMPLETED ✅

### Week 1: Foundation & Parser
- ✅ Added OpenAPI dependencies (openapiv3, jsonschema, serde_yaml)
- ✅ Created src/tools/openapi/ module structure
- ✅ Implemented OpenApiSpec with metadata and validation
- ✅ Implemented OpenApiValidator with quality scoring
- ✅ Implemented SpecHarvester with deduplication
- ✅ Implemented ApisGuruSource for spec loading
- ✅ Added OpenApiConfig to config system
- ✅ Created CLI command: zeroclaw openapi harvest
- ✅ All code compiles successfully

### Week 2: Runtime Executor
- ✅ Created src/tools/openapi/auth.rs with 5 auth providers
- ✅ Created src/tools/openapi/executor.rs with OpenApiTool
- ✅ Created src/tools/openapi/registry.rs for spec/tool management
- ✅ Integrated with main tool system
- ✅ Added CLI commands: list, tools, test, search
- ✅ All code compiles (0 errors, 0 warnings)

### Week 2: Integration & Testing
- ✅ Downloaded APIs.guru repository (4,151 files)
- ✅ Created registry with 50 test specs
- ✅ Verified tool loading: 26 specs loaded, 202 tools generated
- ✅ Tested search functionality: 35 tools found for "account"
- ✅ All CLI commands working

## CURRENT STATUS

**Working System:**
- 26/50 specs loaded successfully (52% success rate)
- 202 tools generated and searchable
- CLI commands functional: harvest, list, tools, test, search
- Tool registry operational

**Known Issues:**
- 24/50 specs failed to parse (Swagger 2.0 specs need conversion)
- Need Swagger 2.0 → OpenAPI 3.0 converter
- Harvest command slow for large datasets (needs optimization)

## TODO - Priority Order

### HIGH PRIORITY (Next 1-2 Days)

1. **Add Swagger 2.0 Support**
   - Add swagger2openapi converter dependency
   - Auto-convert Swagger 2.0 specs during harvest
   - Re-run harvest to get all ~1,900 specs working

2. **Optimize Harvest Performance**
   - Use original file paths instead of copying
   - Parallel spec processing
   - Progress indicators

3. **Test Real API Execution**
   - Pick a working spec (e.g., Adyen Account API)
   - Test actual HTTP request execution
   - Verify auth handling
   - Fix any runtime issues

4. **Add Security Checks**
   - Domain allowlist integration
   - SSRF protection
   - Rate limiting

### MEDIUM PRIORITY (Next 3-7 Days)

5. **Converters**
   - Postman → OpenAPI converter
   - AWS Smithy → OpenAPI (optional)
   - Google Discovery → OpenAPI (optional)

6. **Native Tools (Top 10)**
   - GitHub API
   - Stripe API
   - Slack API
   - Google Drive API
   - Notion API

7. **MCP Server**
   - Create OpenApiMcpServer
   - Add zeroclaw openapi mcp start command
   - Test MCP integration

### LOW PRIORITY (Next 1-2 Weeks)

8. **Documentation**
   - Create docs/OPENAPI.md
   - Add usage examples
   - Document auth configuration
   - Troubleshooting guide

9. **Testing**
   - Unit tests for auth providers
   - Unit tests for executor
   - Integration tests for registry
   - End-to-end tests

10. **Community Pipeline**
    - GitHub Actions for spec validation
    - Automated quality scoring
    - PR submission workflow
    - Community contribution guide

## METRICS

**Current:**
- Specs in registry: 50 (26 loaded, 24 failed)
- Tools generated: 202
- Success rate: 52%
- CLI commands: 5 (harvest, list, tools, test, search)

**Target (Week 3):**
- Specs in registry: 1,900+ (from APIs.guru)
- Tools generated: 5,000-10,000
- Success rate: 80%+
- Converters: 1-3 (Postman, AWS, Google)

**Target (Week 5):**
- Specs in registry: 3,500-4,000
- Tools generated: 10,000-15,000
- Success rate: 85%+
- Native tools: 10
- MCP server: Working
- Documentation: Complete
