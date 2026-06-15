# T4.5 — Key backup/recovery flow verification

**Status:** pending
**Track:** 4 (Cryptographic Identity, Anti-Entropy & UI Hardening)
**Dependencies:** T4.1
**Blocks:** none

## Technical Context
- `crypto/backup.rs` exists (key backup/recovery); AuditEventType has BackupExported/BackupImported
- No evidence of end-to-end test or mobile UX wiring

## Implementation
1. Verification-first: integration test exporting identity+ratchet sessions+contacts to an encrypted backup blob (passphrase-derived key — audit `backup.rs` KDF: must be Argon2id or scrypt, NOT bare blake3 of passphrase; add if missing, this is the one likely real crypto gap)
2. Importing on a fresh IronCore, asserting full conversational continuity (can decrypt next ratchet message)
3. Wire export/import through FFI if not present

## Edge Cases
- Backup of a *registered* device must handle seniority (imported device re-registers, doesn't clone seniority — interaction with `RegistrationState`)
- Partial import (corrupt blob) must be atomic — all-or-nothing with explicit `CorruptionDetected` error (enum variant exists)
- Passphrase KDF parameters must be embedded in blob header for forward-compat

## Verification
- [ ] Roundtrip integration test
- [ ] Tampered-blob test -> CorruptionDetected, no partial state
- [ ] KDF asserted memory-hard (test that derivation takes >100 ms or checks algorithm tag)
- [ ] Audit events emitted both directions
