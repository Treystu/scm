# T5.7 — UniFFI surface contract test (FFI stability gate)

**Status:** pending
**Track:** 5 (CI/CD, FFI Stability & Repo Hygiene)
**Dependencies:** T5.4, T5.5
**Blocks:** T1.1, T4.2, T4.4

## Technical Context
- `core/src/api.udl` + proc-macro exports in `mobile_bridge.rs`/`contacts_bridge.rs`/`blocked_bridge.rs`
- uniffi 0.31. Breaking the surface silently breaks both apps.

## Implementation
1. Snapshot test: check in a canonical copy of the generated `api.kt` and `SCMessengerCore.swift` public-symbol list
2. Extract `fun |class |interface |enum ` signatures via a small script `scripts/ffi_surface.sh` (not full file — symbol list only)
3. CI job diffs freshly generated surface against snapshot and fails on unapproved change
4. Update procedure documented in the script header

## Edge Cases
- uniffi version bumps regenerate cosmetically different code — symbol-list extraction (not byte diff) makes the gate robust
- Two `PlatformBridge` traits exist (G4) — snapshot only the UniFFI one

## Verification
- [ ] CI fails when an agent adds/removes/renames any exported fn/record/enum without updating the snapshot
- [ ] Passes on no-op rebuild
