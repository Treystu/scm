# Prompt: Fable 5 Pre-Test Readiness Audit

Use this prompt to kick off a Fable session (or a `/sprint` cloud worker) whose
sole job is to certify SCMessenger as code-complete and ready for the
device-testing phase — including reviewing the currently open pull request
before it gets merged or built on further. Fable does not write the fixes
itself: it diagnoses, then hands off a plan detailed enough for Sonnet/Haiku
models to execute mechanically.

---

## Prompt text

> You are auditing SCMessenger for release readiness. The master backlog is
> `fable5plan.md` (Tracks T1–T5, tasks `T<track>.<seq>`), each task mirrored
> in `tasks/T<id>/progress.md` with a verification checklist. `CHANGELOG.md`
> currently claims `1.0.0-rc2` is a complete "Fable 5 plan" release with all
> gates green. Your job is to check whether that claim is actually true
> **right now**, after the most recent commits and the open pull request
> below, and to leave the codebase in a state where "ready for testing" is a
> verified fact, not an assertion.
>
> **0. Review the current status and the open PR before anything else.**
> `main` is at `bc9b25e`. There is one open PR you must review in full before
> any of it gets merged or otherwise committed further:
> - **PR #1** — `claude/v1-0-0-code-gaps-7d849x` → `main`
>   (head `1f52b425`), 22 commits, 68 files changed (+11023/-805). Its
>   description claims it closes out T1.4 (WiFi Direct group-owner-intent
>   from battery state, plus a new task `T1.8` it introduces), T4.5 (Argon2id
>   identity backup export/import + audit events), T4.2-adjacent safety-number
>   verification UI, CLI JSON-RPC message-request handling, and touches the
>   progress checklists for `T1.2`, `T1.3`, `T1.4`, `T1.8`, `T2.4`, `T2.5`,
>   `T4.5`, `T5.7` directly.
>   - **Its CI is not green.** Of 16 check runs on the head commit, only
>     "cubic · AI code reviewer" passed and two binding-generation jobs were
>     skipped; everything else — `Test (ubuntu-latest)`, `Lint`, `WASM`,
>     `iOS`, `iOS Build`, all three `Android` ABI builds, `Android Debug APK`,
>     `FFI Surface Contract`, and `Docs` — is either `failure` or
>     `cancelled`, and GitHub reports `mergeable_state: unstable`.
>   - Do not trust the PR description's claims over the CI result. Check out
>     the branch, reproduce each failing job locally (`cargo test
>     --workspace`, `cargo clippy --workspace --all-features -- -D
>     warnings`, `cargo build --target wasm32-unknown-unknown -p
>     scmessenger-wasm`, the Android NDK cross-builds, the iOS build,
>     `scripts/ffi_surface.sh`), and get the actual error for each failure —
>     pull the job logs from the workflow run if reproducing locally doesn't
>     immediately surface the cause.
>   - For each failure, determine: is this a real regression the PR
>     introduced, pre-existing breakage already on `main` that the PR merely
>     exposed, or a CI infrastructure problem (missing secret, runner
>     mismatch, flaky network dependency)? Say which, with evidence, for
>     every single failing job — "flaky" is not an acceptable conclusion
>     without a second run confirming it.
>   - Only after PR #1's real state is understood should you move on to the
>     rest of this audit; its outcome likely changes which `tasks/T*` boxes
>     below are actually still open versus already fixed-but-blocked-on-CI.
>
> **1. Reconcile progress.md against reality.**
> As of `main` (before PR #1 merges), the following tasks still have
> unchecked verification boxes in their `tasks/T*/progress.md`. PR #1 edits
> several of these same files (see §0) — check the versions of these files
> *on the PR branch*, not on `main`, since the PR may have already flipped
> some boxes (correctly or not):
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
> **6. Deliver output in two parts: a short verdict, then an execution-ready
> plan.** This audit is not the implementation pass — Sonnet and Haiku
> models will pick up your output next and execute it, so your job is to do
> all the hard diagnostic thinking now and leave them nothing to guess at.
>
> **6a. Verdict (a few sentences, at the top).** Is the codebase code-complete
> and ready for testing, yes or no, and why — one line per blocking category
> (PR #1 CI, unchecked progress.md items, TODOs, cloud/ hygiene, device-test
> docs). If yes: say so plainly, attach the full verification log, and skip
> 6b.
>
> **6b. Remediation plan — a numbered, ordered list of atomic tasks**, one
> per fix, written so that a Sonnet or Haiku model can execute each task in
> a single turn with zero architectural judgment left to make. You do the
> investigation and the design decisions; they do the mechanical edit. For
> every task, include ALL of:
>   - **Files**: exact path(s) touched.
>   - **Anchor**: the exact function/struct/line range/symbol to change (not
>     "somewhere in the routing module" — the literal `path:line`).
>   - **Change**: an unambiguous description of the edit — prefer literal
>     before/after code or a diff-shaped description over prose like "fix
>     the bug." If multiple approaches are possible, YOU pick one and state
>     it; never leave "consider using X or Y" for the executing model to
>     resolve.
>   - **Why**: one sentence tying it to the failing test/CI job/unchecked
>     box that motivated it — no invented rationale, and no re-litigating
>     design decisions already made in `fable5plan.md`.
>   - **Verify**: the exact command to run to confirm the task in isolation
>     (e.g. `cargo test -p scmessenger-core wifi_aware`), and what output
>     counts as success.
>   - **Done when**: an explicit, checkable condition (a specific checkbox
>     in a specific `tasks/T*/progress.md` flips, a specific CI job goes
>     green, a specific grep returns empty).
>   - Order the list by dependency — e.g., a task that fixes a compile error
>     blocking `cargo test` must precede tasks whose "Verify" step depends on
>     tests running at all; CI-infrastructure fixes precede tasks that rely
>     on CI to confirm them.
>   - If a listed fix is large enough that it doesn't fit "single turn,
>     zero judgment calls" (e.g. a genuinely missing feature, not a bug),
>     break it into an ordered sub-sequence of atomic tasks rather than
>     leaving one big vague task.
> - Update every `tasks/T*/progress.md` checkbox to match verified reality
>   (not intent), and correct `CHANGELOG.md` if its `1.0.0-rc2` claims don't
>   match what you actually verified — but do this as one of the numbered
>   tasks in the plan, not as a side effect, so Sonnet/Haiku can carry it out
>   the same way as everything else.
> - Do not implement the fixes yourself in this pass. Your deliverable is
>   the diagnosis plus the plan; if something in `fable5plan.md` needs a new
>   task entry to track work this audit surfaced, add it there under the
>   appropriate track before handing off.

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
