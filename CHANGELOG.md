# Changelog

All notable changes to SCMessenger will be documented in this file.

## [0.3.4] — Current

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
