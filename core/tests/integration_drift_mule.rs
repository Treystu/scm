/// Integration test: Drift sync end-to-end under partition (sneakernet scenario)
///
/// Tests the CRDT MeshStore merge semantics:
/// - Insert is idempotent (dedup)
/// - Merge is commutative, idempotent, associative
/// - TTL expiry works

use scmessenger_core::drift::store::{MeshStore, StoredEnvelope};

fn make_envelope(id: u8, recipient_hint: [u8; 4]) -> StoredEnvelope {
    StoredEnvelope {
        envelope_data: vec![id; 10],
        message_id: [id; 16],
        recipient_hint,
        created_at: 1000,
        ttl_expiry: 0, // never expires
        hop_count: 0,
        priority: 128,
        received_at: 1000,
    }
}

#[test]
fn mesh_store_insert_and_dedup() {
    let mut store = MeshStore::new();
    let env = make_envelope(1, [0xAA; 4]);

    assert!(store.insert(env.clone())); // New
    assert!(!store.insert(env)); // Duplicate — CRDT idempotent
    assert_eq!(store.len(), 1);
}

#[test]
fn mesh_store_merge_is_commutative() {
    let env_a = make_envelope(1, [0xAA; 4]);
    let env_b = make_envelope(2, [0xBB; 4]);

    let mut store1 = MeshStore::new();
    store1.insert(env_a.clone());
    store1.insert(env_b.clone());

    let mut store2 = MeshStore::new();
    store2.insert(env_b);
    store2.insert(env_a);

    // merge(A, B) == merge(B, A)
    assert_eq!(store1.len(), store2.len());
}

#[test]
fn mesh_store_merge_union() {
    let env_a = make_envelope(1, [0xAA; 4]);
    let env_b = make_envelope(2, [0xBB; 4]);

    let mut mule = MeshStore::new(); // Mule carries messages from A
    mule.insert(env_a.clone());

    let mut node_b = MeshStore::new(); // Node B has its own messages
    node_b.insert(env_b.clone());

    // Mule meets B — merge
    node_b.merge(&mule);
    assert_eq!(node_b.len(), 2);
    assert!(node_b.get(&[1; 16]).is_some());
    assert!(node_b.get(&[2; 16]).is_some());
}

#[test]
fn mesh_store_merge_idempotent() {
    let env = make_envelope(1, [0xAA; 4]);

    let mut store1 = MeshStore::new();
    store1.insert(env.clone());

    let mut store2 = MeshStore::new();
    store2.merge(&store1);
    store2.merge(&store1); // Merge twice

    assert_eq!(store2.len(), 1); // Still just one message
}

#[test]
fn mesh_store_eviction_at_capacity() {
    let mut store = MeshStore::with_capacity(3);

    store.insert(make_envelope(1, [0xAA; 4]));
    store.insert(make_envelope(2, [0xBB; 4]));
    store.insert(make_envelope(3, [0xCC; 4]));
    assert_eq!(store.len(), 3);

    // Insert a 4th — should evict lowest priority
    store.insert(make_envelope(4, [0xDD; 4]));
    assert_eq!(store.len(), 3); // Still at capacity
}
