# T1.2 — Wire WifiAwareTransport into the live core (de-orphan, G1)

**Status:** partial
**Track:** 1 (Native Hardware & Proximity Transport Layer)
**Dependencies:** T1.1
**Blocks:** T1.3

## Technical Context
- `transport/wifi_aware.rs` is complete (state machine, data paths, RSSI bandwidth model, `wire_discovery_callback`) but unreferenced
- Consumer seam: `MeshService.start()` (`mobile_bridge.rs:227`) and the swarm event loop (`transport/swarm.rs`)
- Settings flag already exists: `MeshSettings.wifi_aware_enabled` (api.udl:222 block)

## Implementation
1. Implement a production `WifiAwarePlatformBridge` whose methods forward over the T1.1 FFI plane (publish/subscribe/data-path requests become `PlatformBridge` calls; preferred: fold into generic `transport_control(transport, op, payload)` to keep the surface small)
2. Instantiate `WifiAwareTransport` inside `MeshService.start()` when `wifi_aware_enabled && bridge.is_available()`
3. On `DataPathInfo` confirmation (IP+port), dial that socket via the existing libp2p TCP transport (`SwarmHandle.dial` path used by `SwarmBridge::dial`, `mobile_bridge.rs:2428`) so Noise/Yamux/Gossipsub ride the Aware data path with zero new protocol code
4. PMK derivation: blake3-derive a 32-byte PMK from the DarkBLE group key (`transport/ble/beacon.rs`) so only mesh members can join data paths

## Edge Cases
- Android-only (iOS has no Wi-Fi Aware API — bridge `is_available()` must return false on iOS; `MultipeerTransport.swift` is the iOS analog)
- Android requires `NEARBY_WIFI_DEVICES` (API 31+) / fine-location (<=30) at runtime — already in manifest, but Kotlin bridge must check grant state before `is_available()=true`
- Aware sessions die on Wi-Fi toggle/Doze: `on_network_changed` (existing PlatformBridge callback) must tear down `DataPathActive` state

## Verification
- [ ] `cargo test -p scmessenger-core wifi_aware` (existing 15 tests still pass)
- [ ] New integration test with `MockWifiAwareBridge` proving: discovery event -> `create_data_path` -> dial issued to `SwarmHandle` (assert via command-channel inspection)
- [ ] Kotlin unit test (Robolectric) for permission-gated availability
