PS F:\zeroclaw> git remote -v
origin  https://github.com/zeroclaw-labs/zeroclaw (fetch)
origin  https://github.com/zeroclaw-labs/zeroclaw (push)
PS F:\zeroclaw> git branch -a
* master
  remotes/origin/HEAD -> origin/master
  remotes/origin/channel-matrix
  remotes/origin/chore-remove-linear-app-usage-dev
  remotes/origin/chore/blacksmith-dev-unify-runners
  remotes/origin/chore/bump-0.4.2
  remotes/origin/chore/bump-v0.3.3
  remotes/origin/chore/bump-v0.3.4
  remotes/origin/chore/ci-docs-cleanup
  remotes/origin/chore/clean-root-artifacts
  remotes/origin/chore/i18n-expand-discord-language-set-20260306
  remotes/origin/chore/sync-scoop-template-version
  remotes/origin/chore/version-bump-0.4.1
  remotes/origin/ci/fix-runner-labels
  remotes/origin/ci/remove-hetzner-vms-v2
  remotes/origin/ci/restore-package-manager-syncs
  remotes/origin/codex/issues-2992-3008-2961
  remotes/origin/dependabot/cargo/master/indicatif-0.18.4
  remotes/origin/dependabot/cargo/master/rust-all-0a9022b353
  remotes/origin/dependabot/cargo/master/tokio-tungstenite-0.29.0
  remotes/origin/dependabot/docker/master/distroless/cc-debian13-9c4fe23
  remotes/origin/dependabot/docker/master/docker-all-63cea7e611
  remotes/origin/dependabot/github_actions/master/docker/setup-buildx-action-4.0.0
  remotes/origin/feat/3093-dynamic-node-discovery
  remotes/origin/feat/3095-deferred-mcp-tools
  remotes/origin/feat/3359-docker-shell-variant
  remotes/origin/feat/3396-wecom-channel
  remotes/origin/feat/3403-ack-reactions-config
  remotes/origin/feat/changelog-auto-sync
  remotes/origin/feat/channel-twitter
  remotes/origin/feat/competitive-edge-heartbeat-sessions-caching
  remotes/origin/feat/configurable-delegate-timeouts
  remotes/origin/feat/control-plane
  remotes/origin/feat/crates-io-publish
  remotes/origin/feat/custom-api-path-3125
  remotes/origin/feat/default-enable-web-tools
  remotes/origin/feat/google-workspace-cli
  remotes/origin/feat/hardware-rpi-aardvark-gpio
  remotes/origin/feat/heartbeat-edge-cases-and-agent-loop
  remotes/origin/feat/heartbeat-interval-defaults
  remotes/origin/feat/microsoft365
  remotes/origin/feat/multi-swarm-and-bugfixes
  remotes/origin/feat/openvpn-tunnel
  remotes/origin/feat/preserve-message-draft-3129
  remotes/origin/feat/skill-creation-3825
  remotes/origin/feat/stt-multi-provider
  remotes/origin/feat/tool-descriptions-i18n
  remotes/origin/feat/tui
  remotes/origin/feat/verifiable-intent
  remotes/origin/feature/interactive-session-state
  remotes/origin/feature/terminal-ui
  remotes/origin/fix/3402-heartbeat-stale-context
  remotes/origin/fix/3425-matrix-channel-compilation
  remotes/origin/fix/3430-mcp-client-atomic-32bit
  remotes/origin/fix/3460-context-window-exceeded
  remotes/origin/fix/3463-install-sh-404
  remotes/origin/fix/3647-qq-markdown-messages
  remotes/origin/fix/3674-vision-history-poisoning
  remotes/origin/fix/3687-docker-dummy-binary
  remotes/origin/fix/3688-daemon-sighup-handling
  remotes/origin/fix/3699-cron-once-security-policy
  remotes/origin/fix/3710-matrix-config-missing-cli-default
  remotes/origin/fix/3798-agent-self-correction-loop
  remotes/origin/fix/3802-telegram-image-transfer
  remotes/origin/fix/3819-bootstrap-files-non-tty
  remotes/origin/fix/3868-cron-oneshot-infinite-loop
  remotes/origin/fix/3878-claude-code-context
  remotes/origin/fix/ack-reactions-3834
  remotes/origin/fix/allowed-roots-path-handling-3082
  remotes/origin/fix/anthropic-500-errors-3493
  remotes/origin/fix/beta-release-token
  remotes/origin/fix/boxpin-agent-run
  remotes/origin/fix/cargo-install-package-rename-conflict
  remotes/origin/fix/clean-readme-stale-sections
  remotes/origin/fix/crates-auto-publish-idempotent
  remotes/origin/fix/cron-add-schedule-3860
  remotes/origin/fix/cron-large-future
  remotes/origin/fix/dashboard-pairing-code
  remotes/origin/fix/deferred-mcp-daemon-3826
  remotes/origin/fix/docker-timeout-and-crates-dedup
  remotes/origin/fix/electric-blue-dist
  remotes/origin/fix/fmt-boxpin
  remotes/origin/fix/install-config-3852
  remotes/origin/fix/install-sh-404-link
  remotes/origin/fix/install-stale-build-cache
  remotes/origin/fix/large-futures-boxpin
  remotes/origin/fix/matrix-sdk-recursion-limit
  remotes/origin/fix/mcp-tools-subagents-3069
  remotes/origin/fix/nextcloud-talk-events-3491
  remotes/origin/fix/proactive-context-window-compaction
  remotes/origin/fix/readme-auto-sync
  remotes/origin/fix/readmes-duplicates
  remotes/origin/fix/release-beta-cache-glibc
  remotes/origin/fix/release-pipeline
  remotes/origin/fix/release-stable-glibc-cache
  remotes/origin/fix/release-sync-tweet-and-crates
  remotes/origin/fix/remove-sync-readme-workflow
  remotes/origin/fix/serde-default-missing-fields
  remotes/origin/fix/signal-channel-delivery
  remotes/origin/fix/slack-thread-events-3084
  remotes/origin/fix/stable-release-cache-glibc
  remotes/origin/fix/stable-release-token
  remotes/origin/fix/stable-tweet-deps
  remotes/origin/fix/sync-release-pipeline-ordering
  remotes/origin/fix/tweet-decouple-docker
  remotes/origin/fix/tweet-only-stable-releases
  remotes/origin/fix/tweet-url-truncation
  remotes/origin/fix/web-dashboard-not-found
  remotes/origin/issue-2487-channel-ack-schema-v2
  remotes/origin/issue-2494-feishu-secret-roundtrip
  remotes/origin/issue-2767-multi-agent-routing-dev
  remotes/origin/issue-3082-allowed-roots-direct-paths
  remotes/origin/issue-3088-3098-3115-dev-clean
  remotes/origin/issue-3153-codex-mcp-config
  remotes/origin/master
  remotes/origin/simianastronaut7/relax-skill-scripts
  remotes/origin/test/termux-release-validation
  remotes/origin/version-bump-v0.3.0
  remotes/origin/web-dashboard
  remotes/origin/web-electric-dashboard
  remotes/origin/work-issues/2432-hybrid-memory-reindex
  remotes/origin/work-issues/2442-fix-formatting-violations
  remotes/origin/work-issues/2879-pairing-modal-check-config
  remotes/origin/work-issues/2881-transcription-initial-prompt
  remotes/origin/work-issues/2916-matrix-password-login
  remotes/origin/work-issues/2918-whatsapp-web-media-support
  remotes/origin/work-issues/2963-configurable-pacing-controls
  remotes/origin/work-issues/3011-fix-dashboard-ws-protocols
  remotes/origin/work-issues/3046-enrich-delegate-prompt
  remotes/origin/work-issues/3189-custom-provider-headers
  remotes/origin/work-issues/3262-channel-proxy-support
  remotes/origin/work-issues/3267-skip-pairing-dialog
  remotes/origin/work-issues/3417-fix-tilde-home-dir-expansion
  remotes/origin/work-issues/3474-docker-restart-docs
  remotes/origin/work-issues/3477-fix-matrix-channel-key
  remotes/origin/work-issues/3486-fix-matrix-image-marker
  remotes/origin/work-issues/3487-channel-approval-manager
  remotes/origin/work-issues/3533-fix-utf8-slice-panic
  remotes/origin/work-issues/3544-fix-codex-sse-buffering
  remotes/origin/work-issues/3563-fix-cron-add-nl-security
  remotes/origin/work-issues/3567-allow-commands-bypass-high-risk
  remotes/origin/work-issues/3568-http-request-private-hosts
  remotes/origin/work-issues/3628-surface-tool-failures-in-chat
  remotes/origin/work-issues/8-gateway-cors-security-headers
  remotes/origin/work/notion-integration
  remotes/origin/work/openvpn-tunnel
  remotes/origin/work/project-delivery
  remotes/origin/work/secure-node-comms
  remotes/origin/work/security-ops
  remotes/origin/zeroclaw_homecomming
PS F:\zeroclaw> git status
On branch master
Your branch is behind 'origin/master' by 145 commits, and can be fast-forwarded.
  (use "git pull" to update your local branch)

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        new file:   DX.md
        new file:   ZEROCLAW_DETAILS.md
        new file:   cursed/README.md
        new file:   hexed/README.md

Changes not staged for commit:
  (use "git add/rm <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   DX.md
        deleted:    ZEROCLAW_DETAILS.md

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        TODO.md

PS F:\zeroclaw> git remote set-url origin https://github.com/manfromexistence/zeroclaw
PS F:\zeroclaw> git remote -v
origin  https://github.com/manfromexistence/zeroclaw (fetch)
origin  https://github.com/manfromexistence/zeroclaw (push)
PS F:\zeroclaw> git fetch origin
PS F:\zeroclaw> git branch -a
* master
  remotes/origin/HEAD -> origin/master
  remotes/origin/channel-matrix
  remotes/origin/chore-remove-linear-app-usage-dev
  remotes/origin/chore/blacksmith-dev-unify-runners
  remotes/origin/chore/bump-0.4.2
  remotes/origin/chore/bump-v0.3.3
  remotes/origin/chore/bump-v0.3.4
  remotes/origin/chore/ci-docs-cleanup
  remotes/origin/chore/clean-root-artifacts
  remotes/origin/chore/i18n-expand-discord-language-set-20260306
  remotes/origin/chore/sync-scoop-template-version
  remotes/origin/chore/version-bump-0.4.1
  remotes/origin/ci/fix-runner-labels
  remotes/origin/ci/remove-hetzner-vms-v2
  remotes/origin/ci/restore-package-manager-syncs
  remotes/origin/codex/issues-2992-3008-2961
  remotes/origin/dependabot/cargo/master/indicatif-0.18.4
  remotes/origin/dependabot/cargo/master/rust-all-0a9022b353
  remotes/origin/dependabot/cargo/master/tokio-tungstenite-0.29.0
  remotes/origin/dependabot/docker/master/distroless/cc-debian13-9c4fe23
  remotes/origin/dependabot/docker/master/docker-all-63cea7e611
  remotes/origin/dependabot/github_actions/master/docker/setup-buildx-action-4.0.0
  remotes/origin/feat/3093-dynamic-node-discovery
  remotes/origin/feat/3095-deferred-mcp-tools
  remotes/origin/feat/3359-docker-shell-variant
  remotes/origin/feat/3396-wecom-channel
  remotes/origin/feat/3403-ack-reactions-config
  remotes/origin/feat/changelog-auto-sync
  remotes/origin/feat/channel-twitter
  remotes/origin/feat/competitive-edge-heartbeat-sessions-caching
  remotes/origin/feat/configurable-delegate-timeouts
  remotes/origin/feat/control-plane
  remotes/origin/feat/crates-io-publish
  remotes/origin/feat/custom-api-path-3125
  remotes/origin/feat/default-enable-web-tools
  remotes/origin/feat/google-workspace-cli
  remotes/origin/feat/hardware-rpi-aardvark-gpio
  remotes/origin/feat/heartbeat-edge-cases-and-agent-loop
  remotes/origin/feat/heartbeat-interval-defaults
  remotes/origin/feat/microsoft365
  remotes/origin/feat/multi-swarm-and-bugfixes
  remotes/origin/feat/openvpn-tunnel
  remotes/origin/feat/preserve-message-draft-3129
  remotes/origin/feat/skill-creation-3825
  remotes/origin/feat/stt-multi-provider
  remotes/origin/feat/tool-descriptions-i18n
  remotes/origin/feat/tui
  remotes/origin/feat/verifiable-intent
  remotes/origin/feature/interactive-session-state
  remotes/origin/feature/terminal-ui
  remotes/origin/fix/3402-heartbeat-stale-context
  remotes/origin/fix/3425-matrix-channel-compilation
  remotes/origin/fix/3430-mcp-client-atomic-32bit
  remotes/origin/fix/3460-context-window-exceeded
  remotes/origin/fix/3463-install-sh-404
  remotes/origin/fix/3647-qq-markdown-messages
  remotes/origin/fix/3674-vision-history-poisoning
  remotes/origin/fix/3687-docker-dummy-binary
  remotes/origin/fix/3688-daemon-sighup-handling
  remotes/origin/fix/3699-cron-once-security-policy
  remotes/origin/fix/3710-matrix-config-missing-cli-default
  remotes/origin/fix/3798-agent-self-correction-loop
  remotes/origin/fix/3802-telegram-image-transfer
  remotes/origin/fix/3819-bootstrap-files-non-tty
  remotes/origin/fix/3868-cron-oneshot-infinite-loop
  remotes/origin/fix/3878-claude-code-context
  remotes/origin/fix/ack-reactions-3834
  remotes/origin/fix/allowed-roots-path-handling-3082
  remotes/origin/fix/anthropic-500-errors-3493
  remotes/origin/fix/beta-release-token
  remotes/origin/fix/boxpin-agent-run
  remotes/origin/fix/cargo-install-package-rename-conflict
  remotes/origin/fix/clean-readme-stale-sections
  remotes/origin/fix/crates-auto-publish-idempotent
  remotes/origin/fix/cron-add-schedule-3860
  remotes/origin/fix/cron-large-future
  remotes/origin/fix/dashboard-pairing-code
  remotes/origin/fix/deferred-mcp-daemon-3826
  remotes/origin/fix/docker-timeout-and-crates-dedup
  remotes/origin/fix/electric-blue-dist
  remotes/origin/fix/fmt-boxpin
  remotes/origin/fix/install-config-3852
  remotes/origin/fix/install-sh-404-link
  remotes/origin/fix/install-stale-build-cache
  remotes/origin/fix/large-futures-boxpin
  remotes/origin/fix/matrix-sdk-recursion-limit
  remotes/origin/fix/mcp-tools-subagents-3069
  remotes/origin/fix/nextcloud-talk-events-3491
  remotes/origin/fix/proactive-context-window-compaction
  remotes/origin/fix/readme-auto-sync
  remotes/origin/fix/readmes-duplicates
  remotes/origin/fix/release-beta-cache-glibc
  remotes/origin/fix/release-pipeline
  remotes/origin/fix/release-stable-glibc-cache
  remotes/origin/fix/release-sync-tweet-and-crates
  remotes/origin/fix/remove-sync-readme-workflow
  remotes/origin/fix/serde-default-missing-fields
  remotes/origin/fix/signal-channel-delivery
  remotes/origin/fix/slack-thread-events-3084
  remotes/origin/fix/stable-release-cache-glibc
  remotes/origin/fix/stable-release-token
  remotes/origin/fix/stable-tweet-deps
  remotes/origin/fix/sync-release-pipeline-ordering
  remotes/origin/fix/tweet-decouple-docker
  remotes/origin/fix/tweet-only-stable-releases
  remotes/origin/fix/tweet-url-truncation
  remotes/origin/fix/web-dashboard-not-found
  remotes/origin/issue-2487-channel-ack-schema-v2
  remotes/origin/issue-2494-feishu-secret-roundtrip
  remotes/origin/issue-2767-multi-agent-routing-dev
  remotes/origin/issue-3082-allowed-roots-direct-paths
  remotes/origin/issue-3088-3098-3115-dev-clean
  remotes/origin/issue-3153-codex-mcp-config
  remotes/origin/master
  remotes/origin/simianastronaut7/relax-skill-scripts
  remotes/origin/test/termux-release-validation
  remotes/origin/version-bump-v0.3.0
  remotes/origin/web-dashboard
  remotes/origin/web-electric-dashboard
  remotes/origin/work-issues/2432-hybrid-memory-reindex
  remotes/origin/work-issues/2442-fix-formatting-violations
  remotes/origin/work-issues/2879-pairing-modal-check-config
  remotes/origin/work-issues/2881-transcription-initial-prompt
  remotes/origin/work-issues/2916-matrix-password-login
  remotes/origin/work-issues/2918-whatsapp-web-media-support
  remotes/origin/work-issues/2963-configurable-pacing-controls
  remotes/origin/work-issues/3011-fix-dashboard-ws-protocols
  remotes/origin/work-issues/3046-enrich-delegate-prompt
  remotes/origin/work-issues/3189-custom-provider-headers
  remotes/origin/work-issues/3262-channel-proxy-support
  remotes/origin/work-issues/3267-skip-pairing-dialog
  remotes/origin/work-issues/3417-fix-tilde-home-dir-expansion
  remotes/origin/work-issues/3474-docker-restart-docs
  remotes/origin/work-issues/3477-fix-matrix-channel-key
  remotes/origin/work-issues/3486-fix-matrix-image-marker
  remotes/origin/work-issues/3487-channel-approval-manager
  remotes/origin/work-issues/3533-fix-utf8-slice-panic
  remotes/origin/work-issues/3544-fix-codex-sse-buffering
  remotes/origin/work-issues/3563-fix-cron-add-nl-security
  remotes/origin/work-issues/3567-allow-commands-bypass-high-risk
  remotes/origin/work-issues/3568-http-request-private-hosts
  remotes/origin/work-issues/3628-surface-tool-failures-in-chat
  remotes/origin/work-issues/8-gateway-cors-security-headers
  remotes/origin/work/backup-recovery-v2
  remotes/origin/work/capability-tool-access
  remotes/origin/work/cloud-ops-v2
  remotes/origin/work/microsoft365
  remotes/origin/work/multi-client-workspaces
  remotes/origin/work/nevis-iam
  remotes/origin/work/notion-integration
  remotes/origin/work/openvpn-tunnel
  remotes/origin/work/project-delivery
  remotes/origin/work/secure-node-comms
  remotes/origin/work/security-ops
  remotes/origin/zeroclaw_homecomming
PS F:\zeroclaw> git stash push -u -m "local-changes-before-merge"
Saved working directory and index state On master: local-changes-before-merge
PS F:\zeroclaw> git pull origin master
From https://github.com/manfromexistence/zeroclaw
 * branch              master     -> FETCH_HEAD
Updating 813ae17f..92940a3d
Fast-forward
 .cargo/audit.toml                                  |   10 +
 .dockerignore                                      |    9 +
 .github/workflows/cross-platform-build-manual.yml  |    2 +-
 .github/workflows/pub-aur.yml                      |  169 ++
 .github/workflows/pub-homebrew-core.yml            |  206 +++
 .github/workflows/pub-scoop.yml                    |  165 ++
 .github/workflows/publish-crates-auto.yml          |   13 +-
 .github/workflows/publish-crates.yml               |   12 +-
 .github/workflows/release-beta-on-push.yml         |   73 +-
 .github/workflows/release-stable-manual.yml        |   97 +-
 .github/workflows/tweet-release.yml                |   75 +-
 .gitignore                                         |    4 +-
 Cargo.lock                                         | 1301 ++++++++++++++-
 Cargo.toml                                         |   14 +-
 Dockerfile                                         |   66 +-
 Dockerfile.ci                                      |   25 +
 Dockerfile.debian                                  |   63 +-
 Dockerfile.debian.ci                               |   34 +
 README.ar.md                                       |    7 +-
 README.bn.md                                       |    7 +-
 README.cs.md                                       |    7 +-
 README.da.md                                       |    7 +-
 README.de.md                                       |    7 +-
 README.el.md                                       |    7 +-
 README.es.md                                       |    7 +-
 README.fi.md                                       |    7 +-
 README.fr.md                                       |    7 +-
 README.he.md                                       |    7 +-
 README.hi.md                                       |    7 +-
 README.hu.md                                       |    7 +-
 README.id.md                                       |    7 +-
 README.it.md                                       |    7 +-
 README.ja.md                                       |    7 +-
 README.ko.md                                       |    7 +-
 README.md                                          |    7 +-
 README.nb.md                                       |    7 +-
 README.nl.md                                       |    7 +-
 README.pl.md                                       |    7 +-
 README.pt.md                                       |    7 +-
 README.ro.md                                       |    7 +-
 README.ru.md                                       |    7 +-
 README.sv.md                                       |    7 +-
 README.th.md                                       |    7 +-
 README.tl.md                                       |    7 +-
 README.tr.md                                       |    7 +-
 README.uk.md                                       |    7 +-
 README.ur.md                                       |    7 +-
 README.vi.md                                       |    7 +-
 README.zh-CN.md                                    |    7 +-
 build.rs                                           |   55 +-
 deny.toml                                          |    7 +
 dist/aur/.SRCINFO                                  |   16 +
 dist/aur/PKGBUILD                                  |   32 +
 dist/scoop/zeroclaw.json                           |   27 +
 docker-compose.yml                                 |    9 +-
 docs/contributing/ci-map.md                        |   22 +-
 docs/contributing/release-process.md               |   37 +
 docs/hardware/arduino-uno-q-setup.md               |    2 +-
 docs/i18n/README.md                                |    7 +-
 docs/i18n/el/config-reference.md                   |  114 --
 docs/i18n/vi/README.md                             |   94 --
 docs/i18n/vi/SUMMARY.md                            |   78 -
 docs/i18n/vi/actions-source-policy.md              |   95 --
 docs/i18n/vi/adding-boards-and-tools.md            |  116 --
 docs/i18n/vi/agnostic-security.md                  |  355 ----
 docs/i18n/vi/arduino-uno-q-setup.md                |  217 ---
 docs/i18n/vi/audit-logging.md                      |  192 ---
 docs/i18n/vi/channels-reference.md                 |  424 -----
 docs/i18n/vi/ci-map.md                             |  125 --
 docs/i18n/vi/commands-reference.md                 |  159 --
 docs/i18n/vi/config-reference.md                   |  521 ------
 docs/i18n/vi/contributing/README.md                |   18 -
 docs/i18n/vi/custom-providers.md                   |  111 --
 docs/i18n/vi/datasheets/arduino-uno.md             |   37 -
 docs/i18n/vi/datasheets/esp32.md                   |   22 -
 docs/i18n/vi/datasheets/nucleo-f401re.md           |   16 -
 docs/i18n/vi/frictionless-security.md              |  312 ----
 docs/i18n/vi/getting-started/README.md             |   29 -
 docs/i18n/vi/hardware-peripherals-design.md        |  324 ----
 docs/i18n/vi/hardware/README.md                    |   19 -
 docs/i18n/vi/langgraph-integration.md              |  239 ---
 docs/i18n/vi/matrix-e2ee-guide.md                  |  141 --
 docs/i18n/vi/mattermost-setup.md                   |   63 -
 docs/i18n/vi/network-deployment.md                 |  206 ---
 docs/i18n/vi/nucleo-setup.md                       |  147 --
 docs/i18n/vi/one-click-bootstrap.md                |  120 --
 docs/i18n/vi/operations-runbook.md                 |  128 --
 docs/i18n/vi/operations/README.md                  |   24 -
 docs/i18n/vi/pr-workflow.md                        |  366 -----
 docs/i18n/vi/project/README.md                     |   17 -
 docs/i18n/vi/providers-reference.md                |  253 ---
 docs/i18n/vi/proxy-agent-playbook.md               |  229 ---
 docs/i18n/vi/reference/README.md                   |   22 -
 docs/i18n/vi/release-process.md                    |  133 --
 docs/i18n/vi/resource-limits.md                    |  109 --
 docs/i18n/vi/reviewer-playbook.md                  |  191 ---
 docs/i18n/vi/sandboxing.md                         |  200 ---
 docs/i18n/vi/security-roadmap.md                   |  188 ---
 docs/i18n/vi/security/README.md                    |   22 -
 docs/i18n/vi/troubleshooting.md                    |  236 ---
 docs/i18n/vi/zai-glm-setup.md                      |  142 --
 docs/openai-temperature-compatibility.md           |   73 +
 docs/ops/operations-runbook.md                     |   58 +
 docs/setup-guides/one-click-bootstrap.md           |   97 ++
 .../specs/2026-03-13-linkedin-tool-design.md       |  314 ++++
 example-plugin/Cargo.toml                          |   12 +
 example-plugin/manifest.toml                       |    8 +
 example-plugin/src/lib.rs                          |   42 +
 install.sh                                         |  206 ++-
 src/agent/agent.rs                                 |  260 ++-
 src/agent/dispatcher.rs                            |   13 +-
 src/agent/loop_.rs                                 |  697 ++++++--
 src/agent/memory_loader.rs                         |   24 +-
 src/agent/tests.rs                                 |    8 +-
 src/approval/mod.rs                                |   15 +
 src/channels/bluesky.rs                            |  571 +++++++
 src/channels/discord.rs                            |   42 +-
 src/channels/mochat.rs                             |  326 ++++
 src/channels/mod.rs                                |  467 +++++-
 src/channels/nextcloud_talk.rs                     |  241 ++-
 src/channels/qq.rs                                 |   43 +-
 src/channels/reddit.rs                             |  504 ++++++
 src/channels/session_backend.rs                    |  108 ++
 src/channels/session_sqlite.rs                     |  558 +++++++
 src/channels/session_store.rs                      |  111 ++
 src/channels/telegram.rs                           |  124 +-
 src/channels/transcription.rs                      |  834 +++++++++-
 src/channels/tts.rs                                |    1 +
 src/channels/twitter.rs                            |  485 ++++++
 src/channels/webhook.rs                            |  409 +++++
 src/channels/whatsapp_web.rs                       |  388 ++++-
 src/commands/mod.rs                                |    2 +
 src/commands/self_test.rs                          |  281 ++++
 src/commands/update.rs                             |  276 ++++
 src/config/mod.rs                                  |   37 +-
 src/config/schema.rs                               | 1190 +++++++++++++-
 src/cron/mod.rs                                    |    5 +-
 src/cron/scheduler.rs                              |   18 +-
 src/cron/store.rs                                  |   84 +-
 src/cron/types.rs                                  |   67 +-
 src/daemon/mod.rs                                  |  194 ++-
 src/gateway/api.rs                                 |   70 +
 src/gateway/api_pairing.rs                         |  383 +++++
 src/gateway/api_plugins.rs                         |   77 +
 src/gateway/mod.rs                                 |  215 ++-
 src/gateway/ws.rs                                  |  228 ++-
 src/heartbeat/engine.rs                            |  177 ++
 src/heartbeat/mod.rs                               |    1 +
 src/heartbeat/store.rs                             |  305 ++++
 src/integrations/registry.rs                       |   77 +-
 src/lib.rs                                         |    4 +
 src/main.rs                                        |  248 ++-
 src/memory/consolidation.rs                        |   20 +-
 src/memory/knowledge_graph.rs                      |  824 ++++++++++
 src/memory/mod.rs                                  |   26 +
 src/memory/response_cache.rs                       |  151 +-
 src/memory/traits.rs                               |   30 +-
 src/multimodal.rs                                  |   24 +
 src/observability/log.rs                           |   74 +
 src/observability/noop.rs                          |   35 +
 src/observability/otel.rs                          |  151 ++
 src/observability/prometheus.rs                    |  184 +++
 src/observability/traits.rs                        |   98 ++
 src/observability/verbose.rs                       |   18 +
 src/onboard/wizard.rs                              |   45 +-
 src/peripherals/mod.rs                             |    2 +-
 src/plugins/error.rs                               |   33 +
 src/plugins/host.rs                                |  325 ++++
 src/plugins/mod.rs                                 |   76 +
 src/plugins/wasm_channel.rs                        |   44 +
 src/plugins/wasm_tool.rs                           |   63 +
 src/providers/anthropic.rs                         |    6 +
 src/providers/azure_openai.rs                      |    3 +
 src/providers/bedrock.rs                           |    2 +
 src/providers/claude_code.rs                       |  476 ++++++
 src/providers/compatible.rs                        |   57 +
 src/providers/copilot.rs                           |    1 +
 src/providers/gemini.rs                            |    1 +
 src/providers/gemini_cli.rs                        |  326 ++++
 src/providers/kilocli.rs                           |  326 ++++
 src/providers/mod.rs                               |  140 +-
 src/providers/ollama.rs                            |    2 +
 src/providers/openai.rs                            |  175 +-
 src/providers/openai_codex.rs                      |  132 +-
 src/providers/openrouter.rs                        |    3 +
 src/providers/reliable.rs                          |  293 +++-
 src/providers/traits.rs                            |   11 +
 src/security/audit.rs                              |  363 +++-
 src/security/pairing.rs                            |   21 +-
 src/skills/audit.rs                                |   46 +-
 src/skills/mod.rs                                  |  225 ++-
 src/tools/browser_delegate.rs                      |  757 +++++++++
 src/tools/cli_discovery.rs                         |   26 +
 src/tools/cron_add.rs                              |  228 ++-
 src/tools/cron_update.rs                           |  206 ++-
 src/tools/delegate.rs                              |    2 +
 src/tools/google_workspace.rs                      |  716 ++++++++
 src/tools/knowledge_tool.rs                        |  581 +++++++
 src/tools/linkedin.rs                              |  804 +++++++++
 src/tools/linkedin_client.rs                       | 1726 ++++++++++++++++++++
 src/tools/mcp_deferred.rs                          |  210 ++-
 src/tools/mod.rs                                   |  126 ++
 src/tools/model_routing_config.rs                  |  136 +-
 src/tools/model_switch.rs                          |  264 +++
 src/tools/proxy_config.rs                          |   12 +-
 src/tools/schedule.rs                              |    9 +-
 src/tools/tool_search.rs                           |   88 +-
 tests/component/config_schema.rs                   |   23 +-
 tests/integration/mod.rs                           |    1 +
 tests/integration/telegram_finalize_draft.rs       |  208 +++
 tests/live/openai_codex_vision_e2e.rs              |    1 +
 tests/support/helpers.rs                           |    7 +-
 tests/support/mock_provider.rs                     |    2 +
 web/.gitignore                                     |    3 +-
 web/public/logo.png                                |  Bin 2156411 -> 0 bytes
 web/src/App.tsx                                    |  103 +-
 web/src/components/layout/Header.tsx               |    4 +-
 web/src/components/layout/Layout.tsx               |   12 +-
 web/src/hooks/useDevices.ts                        |   44 +
 web/src/lib/api.ts                                 |    8 +
 web/src/lib/i18n.ts                                |  747 +++++++--
 web/src/lib/ws.ts                                  |    4 +-
 web/src/pages/AgentChat.tsx                        |   19 +-
 web/src/pages/Config.tsx                           |   18 +-
 web/src/pages/Cost.tsx                             |   33 +-
 web/src/pages/Cron.tsx                             |   55 +-
 web/src/pages/Dashboard.tsx                        |   35 +-
 web/src/pages/Doctor.tsx                           |   21 +-
 web/src/pages/Integrations.tsx                     |   18 +-
 web/src/pages/Logs.tsx                             |   21 +-
 web/src/pages/Memory.tsx                           |   49 +-
 web/src/pages/Pairing.tsx                          |  174 ++
 web/src/pages/Tools.tsx                            |   21 +-
 233 files changed, 24909 insertions(+), 7746 deletions(-)
 create mode 100644 .cargo/audit.toml
 create mode 100644 .github/workflows/pub-aur.yml
 create mode 100644 .github/workflows/pub-homebrew-core.yml
 create mode 100644 .github/workflows/pub-scoop.yml
 create mode 100644 Dockerfile.ci
 create mode 100644 Dockerfile.debian.ci
 create mode 100644 dist/aur/.SRCINFO
 create mode 100644 dist/aur/PKGBUILD
 create mode 100644 dist/scoop/zeroclaw.json
 delete mode 100644 docs/i18n/el/config-reference.md
 delete mode 100644 docs/i18n/vi/README.md
 delete mode 100644 docs/i18n/vi/SUMMARY.md
 delete mode 100644 docs/i18n/vi/actions-source-policy.md
 delete mode 100644 docs/i18n/vi/adding-boards-and-tools.md
 delete mode 100644 docs/i18n/vi/agnostic-security.md
 delete mode 100644 docs/i18n/vi/arduino-uno-q-setup.md
 delete mode 100644 docs/i18n/vi/audit-logging.md
 delete mode 100644 docs/i18n/vi/channels-reference.md
 delete mode 100644 docs/i18n/vi/ci-map.md
 delete mode 100644 docs/i18n/vi/commands-reference.md
 delete mode 100644 docs/i18n/vi/config-reference.md
 delete mode 100644 docs/i18n/vi/contributing/README.md
 delete mode 100644 docs/i18n/vi/custom-providers.md
 delete mode 100644 docs/i18n/vi/datasheets/arduino-uno.md
 delete mode 100644 docs/i18n/vi/datasheets/esp32.md
 delete mode 100644 docs/i18n/vi/datasheets/nucleo-f401re.md
 delete mode 100644 docs/i18n/vi/frictionless-security.md
 delete mode 100644 docs/i18n/vi/getting-started/README.md
 delete mode 100644 docs/i18n/vi/hardware-peripherals-design.md
 delete mode 100644 docs/i18n/vi/hardware/README.md
 delete mode 100644 docs/i18n/vi/langgraph-integration.md
 delete mode 100644 docs/i18n/vi/matrix-e2ee-guide.md
 delete mode 100644 docs/i18n/vi/mattermost-setup.md
 delete mode 100644 docs/i18n/vi/network-deployment.md
 delete mode 100644 docs/i18n/vi/nucleo-setup.md
 delete mode 100644 docs/i18n/vi/one-click-bootstrap.md
 delete mode 100644 docs/i18n/vi/operations-runbook.md
 delete mode 100644 docs/i18n/vi/operations/README.md
 delete mode 100644 docs/i18n/vi/pr-workflow.md
 delete mode 100644 docs/i18n/vi/project/README.md
 delete mode 100644 docs/i18n/vi/providers-reference.md
 delete mode 100644 docs/i18n/vi/proxy-agent-playbook.md
 delete mode 100644 docs/i18n/vi/reference/README.md
 delete mode 100644 docs/i18n/vi/release-process.md
 delete mode 100644 docs/i18n/vi/resource-limits.md
 delete mode 100644 docs/i18n/vi/reviewer-playbook.md
 delete mode 100644 docs/i18n/vi/sandboxing.md
 delete mode 100644 docs/i18n/vi/security-roadmap.md
 delete mode 100644 docs/i18n/vi/security/README.md
 delete mode 100644 docs/i18n/vi/troubleshooting.md
 delete mode 100644 docs/i18n/vi/zai-glm-setup.md
 create mode 100644 docs/openai-temperature-compatibility.md
 create mode 100644 docs/superpowers/specs/2026-03-13-linkedin-tool-design.md
 create mode 100644 example-plugin/Cargo.toml
 create mode 100644 example-plugin/manifest.toml
 create mode 100644 example-plugin/src/lib.rs
 create mode 100644 src/channels/bluesky.rs
 create mode 100644 src/channels/mochat.rs
 create mode 100644 src/channels/reddit.rs
 create mode 100644 src/channels/session_backend.rs
 create mode 100644 src/channels/session_sqlite.rs
 create mode 100644 src/channels/twitter.rs
 create mode 100644 src/channels/webhook.rs
 create mode 100644 src/commands/mod.rs
 create mode 100644 src/commands/self_test.rs
 create mode 100644 src/commands/update.rs
 create mode 100644 src/gateway/api_pairing.rs
 create mode 100644 src/gateway/api_plugins.rs
 create mode 100644 src/heartbeat/store.rs
 create mode 100644 src/memory/knowledge_graph.rs
 create mode 100644 src/plugins/error.rs
 create mode 100644 src/plugins/host.rs
 create mode 100644 src/plugins/mod.rs
 create mode 100644 src/plugins/wasm_channel.rs
 create mode 100644 src/plugins/wasm_tool.rs
 create mode 100644 src/providers/claude_code.rs
 create mode 100644 src/providers/gemini_cli.rs
 create mode 100644 src/providers/kilocli.rs
 create mode 100644 src/tools/browser_delegate.rs
 create mode 100644 src/tools/google_workspace.rs
 create mode 100644 src/tools/knowledge_tool.rs
 create mode 100644 src/tools/linkedin.rs
 create mode 100644 src/tools/linkedin_client.rs
 create mode 100644 src/tools/model_switch.rs
 create mode 100644 tests/integration/telegram_finalize_draft.rs
 delete mode 100644 web/public/logo.png
 create mode 100644 web/src/hooks/useDevices.ts
 create mode 100644 web/src/pages/Pairing.tsx
PS F:\zeroclaw> git stash pop
On branch master
Your branch is up to date with 'origin/master'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        new file:   DX.md
        new file:   ZEROCLAW_DETAILS.md
        new file:   cursed/README.md
        new file:   hexed/README.md

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        TODO.md

Dropped refs/stash@{0} (6a3290b5462849528104a65103eb85c753ae599d)
PS F:\zeroclaw> git add -A && git commit -m "chore: add DX.md, ZEROCLAW_DETAILS.md, cursed/README.md, hexed/README.md, TODO.md"
At line:1 char:12
+ git add -A && git commit -m "chore: add DX.md, ZEROCLAW_DETAILS.md, c ...
+            ~~
The token '&&' is not a valid statement separator in this version.
    + CategoryInfo          : ParserError: (:) [], ParentContainsErrorRecordException
    + FullyQualifiedErrorId : InvalidEndOfLine

PS F:\zeroclaw> git add -A; git commit -m "chore: add DX.md, ZEROCLAW_DETAILS.md, cursed/README.md, hexed/README.md, TODO.md"
[master e5a20779] chore: add DX.md, ZEROCLAW_DETAILS.md, cursed/README.md, hexed/README.md, TODO.md
 5 files changed, 498 insertions(+)
 create mode 100644 DX.md
 create mode 100644 TODO.md
 create mode 100644 ZEROCLAW_DETAILS.md
 create mode 100644 cursed/README.md
 create mode 100644 hexed/README.md
PS F:\zeroclaw> git push origin master
Enumerating objects: 8, done.
Counting objects: 100% (8/8), done.
Delta compression using up to 12 threads
Compressing objects: 100% (5/5), done.
Writing objects: 100% (7/7), 7.25 KiB | 1.45 MiB/s, done.
Total 7 (delta 1), reused 1 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
To https://github.com/manfromexistence/zeroclaw
   92940a3d..e5a20779  master -> master
   