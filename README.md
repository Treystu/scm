# SCMessenger

An autonomous survival mesh messenger for commodity iOS and Android devices. SCMessenger provides delay-tolerant, peer-to-peer communication over BLE and Wi-Fi with zero dependence on ISP infrastructure or the electrical grid. Messages route through a mycorrhizal mesh network that opportunistically forwards data via device-to-device encounters, using cryptographic ratchets for end-to-end encryption and custody-based relay for store-and-forward delivery.

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│  Platform (iOS / Android / CLI / WASM)          │
├─────────────────────────────────────────────────┤
│  Bridge (FFI: mobile_bridge, contacts_bridge)   │
├─────────────────────────────────────────────────┤
│  IronCore (orchestration entry point)           │
├──────────┬──────────┬───────────┬───────────────┤
│  Crypto  │ Routing  │ Transport │  Drift/DTN    │
│ (Ratchet │ (Engine  │ (Swarm,   │  (Sync,       │
│  Session │  Local,  │  BLE,     │   Sketch,     │
│  Manager │  Neigh,  │  Wi-Fi,   │   Store,      │
│  Kani)   │  Global) │  Escal.)  │   Relay)      │
├──────────┴──────────┴───────────┴───────────────┤
│  Store (backend, relay_custody, outbox, dedup)  │
└─────────────────────────────────────────────────┘
```

## Prerequisites

- **Rust stable** (1.75+)
- **Cargo** (included with Rust)
- **Android NDK** — set `ANDROID_NDK_HOME` environment variable
- **Xcode** (for iOS builds, macOS only)

## Quick Start

```bash
# Build the workspace
cargo build

# Run all tests
cargo test

# Run the CLI
cargo run -p scmessenger-cli
```

## Module Map

| Directory    | Description                                              |
|-------------|----------------------------------------------------------|
| `core/`     | Core library — crypto, routing, transport, drift, store  |
| `cli/`      | Command-line interface and local HTTP server              |
| `wasm/`     | WebAssembly target for browser-based clients             |
| `ios/`      | iOS app with native BLE/L2CAP and Multipeer transports   |
| `android/`  | Android app with BLE, Wi-Fi Aware, and Wi-Fi Direct      |

Each module has its own `README.md` with detailed documentation.

## License

MIT — see [LICENSE](LICENSE).
