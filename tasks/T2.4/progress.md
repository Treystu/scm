# T2.4 — Background sync scheduling on both platforms

**Status:** pending
**Track:** 2 (Asynchronous Storage & Delay-Tolerant Networking)
**Dependencies:** T2.3, T1.6
**Blocks:** none

## Technical Context
- iOS `MeshBackgroundService` + BGTaskScheduler ids (registered in Info.plist); Android `MeshForegroundService` + `RECEIVE_BOOT_COMPLETED`
- Core API: `MeshService.pause()/resume()`, Drift `new_drift_sync()` (`iron_core.rs:3027`)

## Implementation
1. iOS: `BGProcessingTaskRequest` (`com.scmessenger.mesh.processing`) handler runs a bounded drift maintenance cycle — new core FFI `run_maintenance_cycle(budget_ms: u32) -> MaintenanceReport` wrapping `drift/relay.rs` maintenance + sweeper, guaranteed to return within budget
2. Android: `WorkManager` periodic job (15 min floor) as belt-and-suspenders alongside the foreground service, calling the same FFI
3. Boot receiver restarts foreground service (receiver exists per manifest — verify it actually starts the service on API 34+ where BOOT_COMPLETED FGS-launch needs `FOREGROUND_SERVICE_DATA_SYNC` type, already declared)

## Edge Cases
- iOS grants processing tasks rarely (often only when charging+idle) — never depend on it for correctness, only opportunistic sync
- Budget enforcement must be cooperative (check elapsed in loop) since Rust can't be preempted
- Android 14 restricts FGS start from BOOT_COMPLETED to specific types — `dataSync` qualifies but verify with targetSdk used

## Verification
- [ ] Rust unit test: `run_maintenance_cycle(50)` returns in <100 ms wall-clock with work remaining flagged in report
- [ ] XCTest registering the BG task handler
- [ ] Android instrumented test (or Robolectric) that boot receiver schedules the service
- [ ] FFI snapshot updated (T5.7)
