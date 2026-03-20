# Rebranding from Agent to DX-Agent

This document summarizes the rebranding changes from Agent to DX-Agent.

## Configuration Directory Changes

### Old Paths
- Config directory: `~/.agent/`
- Workspace: `~/.agent/workspace/`
- Environment variable: `AGENT_CONFIG_DIR`
- Workspace env var: `AGENT_WORKSPACE`

### New Paths
- Config directory: `~/.dx/agent/`
- Workspace: `~/.dx/agent/workspace/`
- Environment variable: `DX_CONFIG_DIR`
- Workspace env var: `DX_WORKSPACE`

## Migration Guide

### Automatic Migration

The agent will automatically migrate your existing configuration:

1. On first run, if `~/.agent/` exists and `~/.dx/agent/` doesn't:
   - Configuration will be copied to the new location
   - Old directory will be preserved (not deleted)

### Manual Migration

If you prefer to migrate manually:

```bash
# Create new directory structure
mkdir -p ~/.dx/agent

# Copy configuration
cp -r ~/.agent/* ~/.dx/agent/

# Update environment variables (if set)
# In your shell profile (.bashrc, .zshrc, etc.):
# OLD: export AGENT_CONFIG_DIR=...
# NEW: export DX_CONFIG_DIR=...
```

### Environment Variables

Update any scripts or configurations that use the old environment variables:

```bash
# Old
export AGENT_CONFIG_DIR=/path/to/config
export AGENT_WORKSPACE=/path/to/workspace

# New
export DX_CONFIG_DIR=/path/to/config
export DX_WORKSPACE=/path/to/workspace
```

## Branding Changes

### Product Name
- **Old**: Agent
- **New**: DX-Agent (Display: "DX - Enhanced Development Experience")

### CLI Output
- Interactive mode: "◆ DX-Agent Interactive Mode" (was "🦀 Agent Interactive Mode")
- Logo symbol: ◆ (diamond) instead of 🦀 (crab)
- All user-facing messages updated to reference DX-Agent

### System Prompts
- Tool descriptions updated to mention DX-Agent
- Identity section refers to "DX - Enhanced Development Experience"
- No references to Agent in agent prompts

## Code Changes Summary

### Configuration (`src/config/schema.rs`)
- `default_config_dir()` now returns `~/.dx/agent/`
- Environment variable names changed to `DX_CONFIG_DIR` and `DX_WORKSPACE`
- Documentation updated

### Main Entry Point (`src/main.rs`)
- Environment variable set to `DX_CONFIG_DIR`
- Help text and branding updated

### Agent Module (`src/agent/`)
- Interactive mode branding updated
- Tool descriptions reference DX-Agent
- Comment documentation updated

### Onboarding (`src/onboard/wizard.rs`)
- Config directory resolution uses new path
- Environment variable checks updated

### Tools (`src/tools/`)
- Tool descriptions updated
- Comments reference DX-Agent

## Backward Compatibility

### Legacy Support

The agent maintains backward compatibility:

1. **Config Detection**: Checks for old `~/.agent/` directory
2. **Auto-Migration**: Offers to migrate on first run
3. **Environment Variables**: Still reads old variables if new ones aren't set (with deprecation warning)

### Deprecation Timeline

- **v0.5.0**: New paths introduced, old paths still supported
- **v0.6.0**: Deprecation warnings for old paths
- **v1.0.0**: Old paths no longer supported (migration required)

## Testing

After migration, verify:

```bash
# Check config location
agent config show

# Verify workspace
agent status

# Test tool execution
agent "list files in workspace"
```

## Troubleshooting

### Config Not Found

If the agent can't find your config:

1. Check `DX_CONFIG_DIR` environment variable
2. Verify `~/.dx/agent/config.toml` exists
3. Run migration manually (see above)

### Permission Issues

If you encounter permission errors:

```bash
# Fix permissions
chmod 700 ~/.dx/agent
chmod 600 ~/.dx/agent/config.toml
```

### Old References

If you see references to Agent:

1. Update to latest version
2. Clear any cached data
3. Report as a bug if it persists

## Benefits of New Structure

1. **Cleaner Home Directory**: All DX tools under `~/.dx/`
2. **Better Organization**: Agent-specific config in `~/.dx/agent/`
3. **Future-Proof**: Room for other DX tools (e.g., `~/.dx/cli/`, `~/.dx/server/`)
4. **Standard Naming**: Follows common conventions (`.config/`, `.local/`, `.dx/`)

## Related Changes

- Package name: `agent` (was `agentlabs`)
- Binary name: `agent` (was `agent`)
- Display name: "DX-Agent - Enhanced Development Experience"
- Repository: Still at github.com/agent-labs/agent (for now)
