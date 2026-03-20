# macOS Update and Uninstall Guide

This page documents supported update and uninstall procedures for Agent on macOS (OS X).

Last verified: **February 22, 2026**.

## 1) Check current install method

```bash
which agent
agent --version
```

Typical locations:

- Homebrew: `/opt/homebrew/bin/agent` (Apple Silicon) or `/usr/local/bin/agent` (Intel)
- Cargo/bootstrap/manual: `~/.cargo/bin/agent`

If both exist, your shell `PATH` order decides which one runs.

## 2) Update on macOS

### A) Homebrew install

```bash
brew update
brew upgrade agent
agent --version
```

### B) Clone + bootstrap install

From your local repository checkout:

```bash
git pull --ff-only
./install.sh --prefer-prebuilt
agent --version
```

If you want source-only update:

```bash
git pull --ff-only
cargo install --path . --force --locked
agent --version
```

### C) Manual prebuilt binary install

Re-run your download/install flow with the latest release asset, then verify:

```bash
agent --version
```

## 3) Uninstall on macOS

### A) Stop and remove background service first

This prevents the daemon from continuing to run after binary removal.

```bash
agent service stop || true
agent service uninstall || true
```

Service artifacts removed by `service uninstall`:

- `~/Library/LaunchAgents/com.agent.daemon.plist`

### B) Remove the binary by install method

Homebrew:

```bash
brew uninstall agent
```

Cargo/bootstrap/manual (`~/.cargo/bin/agent`):

```bash
cargo uninstall agent || true
rm -f ~/.cargo/bin/agent
```

### C) Optional: remove local runtime data

Only run this if you want a full cleanup of config, auth profiles, logs, and workspace state.

```bash
rm -rf ~/.agent
```

## 4) Verify uninstall completed

```bash
command -v agent || echo "agent binary not found"
pgrep -fl agent || echo "No running agent process"
```

If `pgrep` still finds a process, stop it manually and re-check:

```bash
pkill -f agent
```

## Related docs

- [One-Click Bootstrap](one-click-bootstrap.md)
- [Commands Reference](../reference/cli/commands-reference.md)
- [Troubleshooting](../ops/troubleshooting.md)
