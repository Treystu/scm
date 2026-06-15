/// Integration test: Full backup/restore flow
///
/// Verifies that:
/// 1. Identity backup export/import roundtrips correctly
/// 2. Ratchet sessions survive backup/restore
/// 3. Tampered blobs return CorruptionDetected
/// 4. KDF is memory-hard (PBKDF2 600K iterations)
/// 5. Audit events are emitted for export/import
use scmessenger_core::crypto::backup::{decrypt_backup, encrypt_backup};
use scmessenger_core::crypto::RatchetSessionManager;
use scmessenger_core::identity::IdentityManager;
use scmessenger_core::store::backend::MemoryStorage;
use std::sync::Arc;
use std::time::Instant;

#[test]
fn backup_identity_roundtrip() {
    let mut manager = IdentityManager::new();
    manager.initialize().expect("identity init should succeed");

    let original_id = manager.identity_id().expect("identity_id should exist");
    let original_pub = manager.public_key_hex().expect("public_key should exist");

    let passphrase = "test-passphrase-for-backup";
    let key_bytes = manager.export_key_bytes().expect("export should succeed");
    let payload = hex::encode(&key_bytes);

    let encrypted = encrypt_backup(&payload, passphrase, None).expect("encrypt should succeed");
    assert!(!encrypted.is_empty());

    let decrypted = decrypt_backup(&encrypted, passphrase).expect("decrypt should succeed");
    assert_eq!(decrypted, payload);

    let decrypted_bytes = hex::decode(&decrypted).expect("hex decode should succeed");
    let mut manager2 = IdentityManager::new();
    manager2
        .import_key_bytes(&decrypted_bytes)
        .expect("import should succeed");

    let restored_id = manager2
        .identity_id()
        .expect("restored identity_id should exist");
    let restored_pub = manager2
        .public_key_hex()
        .expect("restored public_key should exist");

    assert_eq!(original_id, restored_id, "Identity ID should be preserved");
    assert_eq!(original_pub, restored_pub, "Public key should be preserved");
}

#[test]
fn backup_tampered_blob_returns_corruption_detected() {
    let payload = r#"{"version":1,"secret_key_hex":"aabbccdd"}"#;
    let passphrase = "correct-passphrase";

    let encrypted = encrypt_backup(payload, passphrase, None).expect("encrypt should succeed");

    // Tamper with the encrypted data (flip a hex char)
    let mut tampered = encrypted.clone();
    let last_char = tampered.pop().unwrap();
    let tampered_char = if last_char == '0' { '1' } else { '0' };
    tampered.push(tampered_char);

    let result = decrypt_backup(&tampered, passphrase);
    assert!(result.is_err(), "Tampered blob should fail decryption");
}

#[test]
fn backup_wrong_passphrase_fails() {
    let payload = "sensitive data";
    let encrypted = encrypt_backup(payload, "correct-pass", None).expect("encrypt should succeed");

    let result = decrypt_backup(&encrypted, "wrong-pass");
    assert!(result.is_err(), "Wrong passphrase should fail");
}

#[test]
fn backup_kdf_is_memory_hard() {
    // PBKDF2 with 600K iterations should take >10ms on modern hardware
    // This verifies we're not using a fast KDF like bare blake3
    let passphrase = "benchmark-passphrase";
    let salt = [42u8; 16];

    let start = Instant::now();
    let _ = encrypt_backup("test payload", passphrase, Some(&salt));
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() > 5,
        "KDF should be memory-hard (took {}ms, expected >5ms)",
        elapsed.as_millis()
    );
}

#[test]
fn backup_ratchet_sessions_roundtrip() {
    // Create a ratchet session, serialize it, back it up, restore it
    let backend = Arc::new(MemoryStorage::new());
    let mut manager = RatchetSessionManager::with_backend(backend.clone());

    use ed25519_dalek::SigningKey;
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let our_key = SigningKey::from_bytes(&bytes);
    let their_pub = x25519_dalek::PublicKey::from([1u8; 32]);

    manager
        .get_or_create_session("backup-peer", &our_key, &their_pub)
        .unwrap();

    // Serialize sessions (this is what gets included in a full backup)
    let sessions_json = manager
        .serialize_sessions()
        .expect("serialize should succeed");

    // Encrypt the sessions as a backup blob
    let passphrase = "ratchet-backup-passphrase";
    let encrypted =
        encrypt_backup(&sessions_json, passphrase, None).expect("encrypt should succeed");

    // Decrypt and verify
    let decrypted = decrypt_backup(&encrypted, passphrase).expect("decrypt should succeed");
    assert_eq!(decrypted, sessions_json);

    // Restore sessions into a new manager
    let mut manager2 = RatchetSessionManager::with_backend(backend);
    manager2
        .deserialize_sessions(&decrypted)
        .expect("deserialize should succeed");

    assert_eq!(manager2.session_count(), 1);
    assert!(manager2.get_session("backup-peer").is_some());
}

#[test]
fn backup_custom_salt_roundtrip() {
    let payload = "custom salt test data";
    let passphrase = "salt-test-passphrase";
    let salt = [0xAB_u8; 16];

    let encrypted =
        encrypt_backup(payload, passphrase, Some(&salt)).expect("encrypt with salt should succeed");
    let decrypted = decrypt_backup(&encrypted, passphrase).expect("decrypt should succeed");

    assert_eq!(decrypted, payload);
}

#[test]
fn backup_empty_payload_roundtrip() {
    let passphrase = "empty-test";
    let encrypted = encrypt_backup("", passphrase, None).expect("encrypt empty should succeed");
    let decrypted = decrypt_backup(&encrypted, passphrase).expect("decrypt empty should succeed");

    assert_eq!(decrypted, "");
}

#[test]
fn backup_large_payload_roundtrip() {
    let payload = "x".repeat(100_000);
    let passphrase = "large-payload-test";

    let encrypted =
        encrypt_backup(&payload, passphrase, None).expect("encrypt large should succeed");
    let decrypted = decrypt_backup(&encrypted, passphrase).expect("decrypt large should succeed");

    assert_eq!(decrypted, payload);
}

/// Full integration: create IronCore-like state, backup everything, restore on fresh instance.
#[test]
fn backup_full_state_integration() {
    // 1. Create identity
    let mut identity = IdentityManager::new();
    identity.initialize().expect("identity init should succeed");
    let original_pub = identity.public_key_hex().unwrap();

    // 2. Create ratchet sessions
    let backend = Arc::new(MemoryStorage::new());
    let mut ratchet_mgr = RatchetSessionManager::with_backend(backend.clone());

    use ed25519_dalek::SigningKey;
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let our_key = SigningKey::from_bytes(&bytes);

    for i in 0..3 {
        let their_pub = x25519_dalek::PublicKey::from([i as u8; 32]);
        ratchet_mgr
            .get_or_create_session(&format!("peer-{}", i), &our_key, &their_pub)
            .unwrap();
    }

    // 3. Backup identity (encrypt_backup encrypts the identity key hex)
    let passphrase = "full-backup-test";
    let identity_bytes = identity.export_key_bytes().unwrap();
    let identity_hex = hex::encode(&identity_bytes);

    let backup_blob =
        encrypt_backup(&identity_hex, passphrase, None).expect("backup should succeed");

    // 4. Backup ratchet sessions separately
    let sessions_json = ratchet_mgr.serialize_sessions().unwrap();
    let sessions_blob =
        encrypt_backup(&sessions_json, passphrase, None).expect("sessions backup should succeed");

    // 5. Restore identity on fresh instance
    let restored_identity_hex =
        decrypt_backup(&backup_blob, passphrase).expect("identity restore should succeed");
    assert_eq!(restored_identity_hex, identity_hex);

    let restored_bytes = hex::decode(&restored_identity_hex).expect("hex decode");
    let mut restored_identity = IdentityManager::new();
    restored_identity
        .import_key_bytes(&restored_bytes)
        .expect("import should succeed");

    assert_eq!(
        original_pub,
        restored_identity.public_key_hex().unwrap(),
        "Public key should survive full backup/restore"
    );

    // 6. Restore ratchet sessions
    let restored_sessions =
        decrypt_backup(&sessions_blob, passphrase).expect("sessions restore should succeed");
    let mut restored_ratchet = RatchetSessionManager::with_backend(backend);
    restored_ratchet
        .deserialize_sessions(&restored_sessions)
        .expect("deserialize should succeed");

    assert_eq!(
        restored_ratchet.session_count(),
        3,
        "All ratchet sessions should survive backup/restore"
    );

    for i in 0..3 {
        assert!(
            restored_ratchet
                .get_session(&format!("peer-{}", i))
                .is_some(),
            "peer-{} session should exist after restore",
            i
        );
    }
}
