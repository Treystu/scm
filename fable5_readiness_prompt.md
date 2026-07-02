# Prompt: Fable 5 Pre-Test Readiness Audit

Use this prompt to kick off a Fable session (or a `/sprint` cloud worker) whose
sole job is to certify SCMessenger as code-complete and ready for the
device-testing phase. Do not have it write new features — it audits,
reconciles, and closes out what's already in flight.

---

## Prompt text

> You are auditing SCMessenger for release readiness. The master backlog is
> `fable5plan.md` (Tracks T1–T5, tasks `T<track>.<seq>`), each task mirrored
> in `tasks/T<id>/progress.md` with a verification checklist. `CHANGELOG.md`
> currently claims `1.0.0-rc2` is a complete "Fable 5 plan" release with all
> gates green. Your job is to check whether that claim is actually true
> **right now**, after the most recent commits, and to leave the codebase in
> a state where "ready for testing" is a verified fact, not an assertion.
>
> **1. Reconcile progress.md against reality.**
> As of this audit, the following tasks still have unchecked verification
> boxes in their `tasks/T*/progress.md`:
> - `T1.2` (WifiAwareTransport de-orphaning) — 3 unchecked items, incl.
>   `cargo test -p scmessenger-core wifi_aware` and a `MockWifiAwareBridge`
>   integration test.
> - `T1.3` (Android WifiAwarePlatformBridge) — 2 unchecked (two-device
>   procedure, Robolectric tests).
> - `T1.4` (Wi-Fi Direct Rust transport) — 4 unchecked (unit tests, two-device
>   procedure, clippy, FFI snapshot).
> - `T2.4` (background sync scheduling) — 4 unchecked (maintenance-cycle
>   budget test, XCTest, Android instrumented test, FFI snapshot).
> - `T2.5` (outbox × drift custody convergence) — 3 unchecked
>   (`integration_retry_lifecycle.rs` assertions, state-transition property
>   test).
> - `T4.5` (key backup/recovery) — 4 unchecked (roundtrip test, tampered-blob
>   test, KDF memory-hardness assertion, audit events).
>
> The commits `5df71b9` ("gemini Updates") and `bc9b25e` ("update for cloud
> orchestrator") landed *after* those checklists were last touched, and they
> add `core/tests/integration_wifi_aware.rs`,
> `core/tests/integration_retry_lifecycle.rs`, additions to
> `core/tests/integration_drift_mule.rs`, two new CI workflows
> (`.github/workflows/cross-platform-test.yml`,
> `.github/workflows/ios-build-test.yml`), and a large new `cloud/`
> orchestrator subsystem (Python + Terraform + Docker, currently covered by
> no lint or test gate). For each unchecked box above: determine whether the
> new commits actually satisfy it, run the named verification command
> yourself, and only then flip the checkbox. Do not take the commit messages
> on faith — read the diffs and run the tests.
>
> **2. Full-suite verification.** Run and report pass/fail with output for:
> - `cargo fmt --check`, `cargo clippy --workspace --all-features -- -D
>   warnings`, `cargo deny check`
> - `cargo test --workspace --all-features`
> - `scripts/ffi_surface.sh` snapshot diff (Kotlin + Swift) — flag any drift
>   introduced by the FFI changes in `5df71b9`/`bc9b25e` that wasn't captured
>   in a snapshot update
> - Cross-compile matrix: Android (aarch64/armv7/x86_64 via `cargo ndk`),
>   iOS (`aarch64-apple-ios`, simulator), WASM
>   (`wasm32-unknown-unknown -p scmessenger-wasm`)
> - Android `./gradlew :app:assembleDebug` and the iOS simulator build
> - The two new GitHub Actions workflows added in `bc9b25e` — confirm they
>   actually run green on this branch's HEAD, not just that the YAML parses
>
> **3. Close the loop on `grep -rn "TODO\|FIXME\|unimplemented!\|todo!"
> core/src android ios cli wasm`.** `iron_core.rs`'s async-keepalive TODO was
> reportedly removed by `T2.1` — confirm it's actually gone, and confirm no
> new TODOs crept in via the two "gemini"/"cloud orchestrator" commits.
>
> **4. Audit the new `cloud/` subsystem for hygiene**, since it shipped with
> no lint/test gate of its own: `cloud/orchestrator/__pycache__/*.pyc` files
> were committed (should be `.gitignore`d, not tracked); confirm no secrets
> (API keys, service-account JSON) are embedded in
> `cloud/terraform/*.tf`, `cloud/scripts/*.sh`, or `cloud/worker/*.sh`.
>
> **5. Two-device / physical field procedures.** `docs/device-testing.md`
> exists — confirm it documents runnable procedures for every task above
> that requires physical-device verification (T1.3, T1.4, T1.6, T2.4), and
> that none of them are placeholders.
>
> **6. Deliver a Go/No-Go verdict**, not a status narrative:
> - A definitive list of any items that are still genuinely incomplete, each
>   with file:line evidence and the specific command/test that would close
>   it — no vague "needs more testing."
>   - If none remain: state plainly that the codebase is code-complete and
>     ready for the testing phase, with the full verification log attached.
> - Update every `tasks/T*/progress.md` checkbox to match verified reality
>   (not intent).
> - Correct `CHANGELOG.md` if its `1.0.0-rc2` claims don't match what you
>   actually verified (e.g. if it claims a gate is green that isn't).
> - Do not implement new functionality to close gaps found here — file them
>   back into `fable5plan.md` under the appropriate track/task if they
>   represent real unfinished work, so a follow-up sprint (not this audit)
>   does the implementation.

---

## How to run it

Local Fable session:

```
fable "$(cat fable5_readiness_prompt.md | sed -n '/^## Prompt text/,/^---$/p')"
```

Or via the cloud orchestrator's Telegram bot:

```
/sprint <paste the prompt text block above>
```
