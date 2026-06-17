# T2.5 — Outbox retry × Drift custody convergence audit

**Status:** partial
**Track:** 2 (Asynchronous Storage & Delay-Tolerant Networking)
**Dependencies:** T2.3
**Blocks:** T3.3

## Technical Context
- Two queuing systems coexist: `store/outbox.rs` (QueuedMessage + `SmartRetryManager`, `routing/smart_retry.rs`) and drift custody (T2.3)
- Risk: same message retried over live swarm AND muled via drift -> duplicate sends, double battery cost

## Implementation
1. Define and enforce a single ownership rule in `iron_core.rs` send path (`iron_core.rs:602` is where legacy->drift envelope conversion happens)
2. When `RoutingDecision.primary == NextHop::StoreAndCarry`, message moves to drift custody and is *removed* from active outbox retry (state-marked `InCustody`)
3. A delivery receipt (`integration_receipt_convergence.rs` machinery) clears both
4. Document the state machine in `ARCHITECTURE.md`

## Edge Cases
- Receipt arrives via a different transport than delivery (likely in mesh) — receipt handling is already transport-agnostic by message_id, verify
- Custody->live transition when a direct route appears (routing engine `resume_prefetch.rs`) must atomically re-claim from drift store to outbox without a window where both own it

## Verification
- [ ] New test in `integration_retry_lifecycle.rs`: force StoreAndCarry, assert outbox stops retrying
- [ ] Restore route, assert exactly-one delivery (dedup count 0 at recipient)
- [ ] State-transition property test (no state where both systems own the message)
