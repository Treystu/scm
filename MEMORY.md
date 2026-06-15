# SCMessenger Project Memory

## Project Identity
- **Name:** SCMessenger v1.0.0
- **Goal:** Production-ready autonomous survival mesh — direct P2P over BLE/Wi-Fi proximity transports, mycorrhizal routing, and delay-tolerant data-muling on commodity iOS/Android with zero ISP/grid dependence
- **Language:** Rust core (53k LOC), Kotlin/Compose (Android), Swift (iOS), WASM (browser)
- **License:** MIT

## Workspace Rules
- **SCMessenger_Clean** (this workspace): Gatekeeper-approved only. Production workspace. Deleting this directory removes the entire MiMo Code workspace.
- **SCMessenger** (pre-existing repo): Working workspace for development, consolidation, pruning. Not the source of truth.
- **Gatekeeper:** Only verified code (cargo check + clippy + fmt + test passing) enters Clean workspace.

## Architecture
```
core/          → scmessenger-core: identity, crypto, transport, store, relay, routing, drift
cli/           → scmessenger-cli: headless daemon + web server (127.0.0.1:9002)
wasm/          → scmessenger-wasm: browser thin-client via JSON-RPC
android/       → Kotlin/Compose app (Gradle, minSdk 26, compileSdk 35)
iOS/           → Swift app (Xcode, iOS 17.0 min)
```

## Key Entry Points
- `core/src/iron_core.rs` — IronCore main struct, owns all subsystems
- `core/src/mobile_bridge.rs` — UniFFI bridge for Android/iOS (PlatformBridge trait at line 1436)
- `core/src/api.udl` — UniFFI surface definition
- `core/src/transport/swarm.rs` — libp2p SwarmCommand, SwarmHandle
- `core/src/drift/` — DTN engine, IBLT sketch, CRDT MeshStore

## Verified Gaps (from Fable 5 audit)
| Gap | Description |
|-----|-------------|
| G1 | WifiAwareTransport orphaned — complete but unreferenced |
| G2 | Wi-Fi Direct has no Rust-side transport |
| G3 | PlatformBridge FFI carries only BLE |
| G4 | Duplicate PlatformBridge traits |
| G5 | SwarmHandle async-command path incomplete |
| G6 | 7 ignored NAT tests |
| G7 | No CI/CD |
| G8 | ~1 GB committed build artifacts |
| G9 | Phantom workspace member `mobile` |
| G10 | No root README/CHANGELOG/LICENSE |

## Execution Order (Fable 5 Plan)
Track 5 (CI/hygiene) FIRST → Track 1 (FFI/transport) → Tracks 2-4 in parallel

## Tool Configuration
- **Primary Model:** openrouter/xiaomi/mimo-v2.5-pro (via MiMo Code)
- **Free Fallback:** mimo/mimo-auto (1M context, zero cost)
- **Critical Fallback:** openrouter/anthropic/claude-opus-4.8
- **Agent Pool:** build, plan, compose, rust-coder, mobile-dev, reviewer

## MiMo Code Features in Use
- **Persistent Memory:** This MEMORY.md + checkpoint.md + notes.md
- **Task Tracking:** Tree-shaped T1, T1.1, T1.2… system
- **Subagents:** Native parallel subagent orchestration
- **Compose Mode:** Spec-driven development with Fable 5 plan as spec
- **Goal/Stop:** /goal command for autonomous completion verification
- **Dream/Distill:** Self-improvement from session traces
