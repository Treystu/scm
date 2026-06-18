# Changelog

All notable changes to SCMessenger will be documented in this file.

## [1.0.0-rc2] — 2026-06-17

Release candidate completing the Fable 5 plan. All core subsystems implemented,
Rust gatekeeper suite passes, and Android/iOS/WASM builds are verified.
Includes WiFi Direct/Aware discovery wiring, background sync scheduling, and
identity backup continuity tests contributed by Gemini.

### Verification

- `cargo test --workspace --all-features` — passed
- `cargo fmt --check`, `cargo clippy --workspace --all-features -- -D warnings`, `cargo deny check` — passed
- `scripts/ffi_surface.sh` (Kotlin + Swift snapshots) — passed
- Android debug APK (`./gradlew :app:assembleDebug`) — succeeded
- iOS Simulator build (`xcodebuild -project SCMessenger.xcodeproj -scheme SCMessenger -destination 'generic/platform=iOS Simulator' build`) — succeeded
- WASM build (`cargo build --target wasm32-unknown-unknown -p scmessenger-wasm`) — succeeded

### Subsystems

- **Routing**: Mycorrhizal mesh engine with local, neighborhood, and global strategies; multipath forwarding; reputation scoring; adaptive TTL
- **Drift / DTN**: Delay-tolerant sync with MinHash sketches, custody-based relay store, frame/envelope protocol, rate limiting, and policy-driven forwarding
- **Crypto**: Double Ratchet encryption, session manager, Kani formal proofs, encrypted backup
- **Identity**: Ed25519 key management with persistent identity store
- **Transport**: Swarm management, BLE (GATT, L2CAP, beaconing, scanning), Wi-Fi Aware, escalation pipeline, NAT traversal, health monitoring
- **Storage**: Pluggable backend, relay custody, outbox, deduplication, blocked-list enforcement, inbox sweeper
- **FFI Bridge**: `mobile_bridge`, `contacts_bridge`, `blocked_bridge` with UniFFI definitions (`api.udl`)
- **CLI**: Interactive command-line client with local Axum HTTP server, BLE daemon, and mesh visualization
- **WASM**: Browser-compatible transport layer with daemon bridge and notification manager
- **iOS**: Native app with BLE Central/Peripheral, L2CAP, MultipeerConnectivity, and mDNS service discovery; SmartTransportRouter
- **Android**: Native app with BLE (GATT client/server, scanner, advertiser, L2CAP), Wi-Fi Aware, Wi-Fi Direct, mDNS discovery; SmartTransportRouter

### Deferred

- Acoustic transport — deferred to post-v1.0.0
