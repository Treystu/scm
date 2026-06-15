# T1.3 — Android WifiAwarePlatformBridge native implementation

**Status:** pending
**Track:** 1 (Native Hardware & Proximity Transport Layer)
**Dependencies:** T1.2
**Blocks:** none

## Technical Context
- `android/app/src/main/java/com/scmessenger/android/transport/WifiAwareTransport.kt` exists (publish/subscribe scaffolding) but has no FFI connection
- Target: implement the Kotlin side of T1.2's bridge methods using `WifiAwareManager`/`WifiAwareSession`/`PublishDiscoverySession`/`SubscribeDiscoverySession` + `ConnectivityManager.NetworkRequest` with `WifiAwareNetworkSpecifier` (PMK variant)

## Implementation
1. Wire `attach()` lifecycle to `MeshForegroundService` start/stop
2. On `onServiceDiscovered` -> call core `on_proximity_data_received`-adjacent discovery callback (the T1.2 control channel)
3. On network-available callback with `WifiAwareNetworkInfo` -> report `(ipv6, port)` back to core
4. Use the link-local IPv6 + the peer's announced port from service-info TLV

## Edge Cases
- Aware unavailable on huge swath of devices (`PackageManager.FEATURE_WIFI_AWARE` optional — manifest already `required=false`)
- `WifiAwareManager.isAvailable()` flaps with Wi-Fi state — register `ACTION_WIFI_AWARE_STATE_CHANGED` receiver
- Doze/App Standby suspends sessions: foreground service (already present, `FOREGROUND_SERVICE_CONNECTED_DEVICE`) keeps it alive; battery-optimization exemption is user-prompted, never silently assumed
- IPv6 link-local requires scope-id when dialing — multiaddr must be `/ip6/<addr>%<scope>/tcp/<port>` (verify libp2p multiaddr scope-id support; if unsupported, bind a local TCP proxy socket)

## Verification
- [ ] Instrumented test on two physical Android devices (documented manual procedure in `docs/device-testing.md` + an `adb`-scripted check): both report `DataPathActive`, then `SwarmBridge.get_peers()` on each shows the other's PeerId
- [ ] CI-side: Robolectric tests for state machine, lint passes
