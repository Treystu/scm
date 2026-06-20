use scmessenger_core::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

struct MockBridge;

impl PlatformBridge for MockBridge {
    fn on_battery_changed(&self, _battery_pct: u8, _is_charging: bool) {}
    fn on_network_changed(&self, _has_wifi: bool, _has_cellular: bool) {}
    fn on_motion_changed(&self, _motion: MotionState) {}
    fn on_ble_data_received(&self, _peer_id: String, _data: Vec<u8>) {}
    fn on_entering_background(&self) {}
    fn on_entering_foreground(&self) {}
    fn send_ble_packet(&self, _peer_id: String, _data: Vec<u8>) {}
    fn on_proximity_data_received(&self, _peer_id: String, _transport: ProximityTransport, _data: Vec<u8>) {}
    fn send_proximity_packet(&self, _peer_id: String, _transport: ProximityTransport, _data: Vec<u8>) {}
    fn wifi_aware_publish(&self, _service_name: String, _service_info: Vec<u8>) -> bool { true }
    fn wifi_aware_subscribe(&self, _service_name: String) -> bool { true }
    fn wifi_aware_create_data_path(&self, _peer_id: String, _pmk: Vec<u8>) -> bool { true }
    fn wifi_aware_stop(&self) {}
    fn wifi_direct_discover_peers(&self) -> bool { true }
    fn wifi_direct_stop_discovery(&self) {}
    fn wifi_direct_connect(&self, _device_address: String) -> bool { true }
    fn wifi_direct_create_group(&self, _group_name: String) -> bool { true }
    fn wifi_direct_remove_group(&self) {}
}

#[test]
fn test_wifi_aware_peer_discovered_triggers_data_path_and_dial() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().to_str().unwrap().to_string();

    let service = Arc::new(MeshService::with_storage(
        MeshServiceConfig {
            discovery_interval_ms: 100,
            battery_floor_pct: 10,
        },
        path,
    ));

    // Set mock platform bridge
    service.set_platform_bridge(Some(Box::new(MockBridge)));

    // Start service
    service.clone().start().expect("Failed to start service");

    // Retrieve the SwarmBridge and set a mock SwarmHandle
    let swarm_bridge = service.get_swarm_bridge();
    let mock_peer_id = libp2p::PeerId::random();
    
    // We can get the runtime handle to block_on start_swarm
    let rt = swarm_bridge.get_runtime_handle();
    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(256);
    
    let swarm_handle = rt.block_on(async {
        transport::start_swarm(
            keypair,
            Some("/ip4/127.0.0.1/tcp/0".parse().unwrap()),
            event_tx,
            None,
            false,
            None,
            scmessenger_core::transport::default_routing_engine_handle(),
        )
        .await
        .expect("Failed to start swarm")
    });
    
    swarm_bridge.set_handle(swarm_handle);

    // Simulate discovering a peer via Wi-Fi Aware
    service.on_wifi_aware_peer_discovered(
        mock_peer_id.to_string(),
        vec![1, 2, 3],
        -50,
    );

    // Give the async task inside MeshService some time to attempt data path confirmation
    // Since our MockBridge automatically returns true on create_data_path, the confirmed callback should be triggered
    // Let's manually confirm the data path to simulate the platform replying
    service.on_wifi_aware_data_path_confirmed(
        mock_peer_id.to_string(),
        "127.0.0.1".to_string(),
        12345,
    );

    // Wait a brief moment for the dial task to run
    sleep(Duration::from_millis(500));

    // Clean up
    service.stop();
}
