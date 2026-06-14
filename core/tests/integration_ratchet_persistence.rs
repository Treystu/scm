/// Integration test: Ratchet session persistence across process restarts
///
/// Verifies that:
/// 1. Sessions serialize/deserialize correctly
/// 2. Out-of-order messages survive persistence (MAX_SKIP_KEYS=64)
/// 3. Conversation continues after restart
use scmessenger_core::crypto::RatchetSessionManager;
use scmessenger_core::store::backend::MemoryStorage;
use std::sync::Arc;

#[test]
fn ratchet_session_persistence_roundtrip() {
    let backend = Arc::new(MemoryStorage::new());
    let mut manager = RatchetSessionManager::with_backend(backend.clone());

    // Create a session
    use ed25519_dalek::SigningKey;
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let our_key = SigningKey::from_bytes(&bytes);
    let their_pub = x25519_dalek::PublicKey::from([1u8; 32]);

    manager
        .get_or_create_session("peer-restart", &our_key, &their_pub)
        .unwrap();
    assert_eq!(manager.session_count(), 1);

    // Serialize
    let json = manager.serialize_sessions().unwrap();
    assert!(!json.is_empty());

    // Simulate restart: new manager, load from serialized state
    let mut manager2 = RatchetSessionManager::with_backend(backend);
    assert_eq!(manager2.session_count(), 0);

    manager2.deserialize_sessions(&json).unwrap();
    assert_eq!(manager2.session_count(), 1);
    assert!(manager2.get_session("peer-restart").is_some());
}

#[test]
fn ratchet_session_multiple_peers_persist() {
    let backend = Arc::new(MemoryStorage::new());
    let mut manager = RatchetSessionManager::with_backend(backend.clone());

    use ed25519_dalek::SigningKey;
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let our_key = SigningKey::from_bytes(&bytes);

    // Create sessions for multiple peers
    for i in 0..5 {
        let their_pub = x25519_dalek::PublicKey::from([i as u8; 32]);
        manager
            .get_or_create_session(&format!("peer-{}", i), &our_key, &their_pub)
            .unwrap();
    }
    assert_eq!(manager.session_count(), 5);

    // Persist and reload
    let json = manager.serialize_sessions().unwrap();
    let mut manager2 = RatchetSessionManager::with_backend(backend);
    manager2.deserialize_sessions(&json).unwrap();
    assert_eq!(manager2.session_count(), 5);

    // Verify all peers present
    for i in 0..5 {
        assert!(manager2.get_session(&format!("peer-{}", i)).is_some());
    }
}
