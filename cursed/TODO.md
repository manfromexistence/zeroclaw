# ZeroClaw UI Integration TODO

> Auto-managed by AI. Updated after every completed or failed task.

## In Progress

- [ ] Phase 1: Split wizard.rs into logical modules

## Pending

- [ ] Phase 2: Integrate onboard UI framework into each module
- [ ] Phase 3: Add splash screens and rainbow effects
- [ ] Phase 4: Add train animation
- [ ] Phase 5: Test complete wizard flow
- [ ] Phase 6: Apply UI framework to other modules (skills, integrations, doctor, etc.)

## Completed

## Blocked / Failed

---

## Phase 1 Details: Split wizard.rs

### Step 1.1: Backup original file ✓
- [x] Copy wizard.rs to wizard.rs.backup

### Step 1.2: Analyze structure
- [ ] Read wizard.rs and identify logical sections
- [ ] Map functions to new modules

### Step 1.3: Create new module files
- [ ] src/onboard/mod.rs - Module exports
- [ ] src/onboard/splash.rs - Logo + train animation
- [ ] src/onboard/provider_setup.rs - Provider configuration
- [ ] src/onboard/channel_setup.rs - Channel setup
- [ ] src/onboard/memory_setup.rs - Memory backend
- [ ] src/onboard/hardware_setup.rs - Hardware config
- [ ] src/onboard/project_setup.rs - Project context
- [ ] src/onboard/quick_setup.rs - Non-interactive mode
- [ ] src/onboard/models.rs - Model catalog management
- [ ] src/onboard/helpers.rs - Utility functions

### Step 1.4: Move code to modules
- [ ] Move functions to appropriate modules
- [ ] Update imports
- [ ] Ensure all public APIs remain the same

### Step 1.5: Test
- [ ] cargo build (compile check)
- [ ] cargo run -- onboard --help (runtime check)

---

Started: 2026-03-19
