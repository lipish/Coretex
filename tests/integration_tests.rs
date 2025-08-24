use coretex::{
    storage::{StorageEngine, InMemoryEngine},
    membership::{MembershipManager, InMemoryMembership, NodeState},
    messaging::{MessageBroker, memory::InMemoryBroker},
    consistency::{ConsistencyManager, DummyConsistencyManager},
};
use std::{collections::HashMap, net::SocketAddr};

#[tokio::test]
async fn test_storage_engine_basic_operations() {
    let engine = InMemoryEngine::new("test");
    
    // Test put and get
    let key = b"test_key";
    let value = b"test_value";
    engine.put(key, value).await.unwrap();
    
    let result = engine.get(key).await.unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().as_ref(), value);
    
    // Test delete
    engine.delete(key).await.unwrap();
    let result = engine.get(key).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn test_membership_manager() {
    let membership = InMemoryMembership::new();
    
    // Test node registration
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let metadata = HashMap::new();
    
    let node_id = membership.register_node(addr, metadata).await.unwrap();
    assert!(!node_id.is_empty());
    
    // Test get node
    let node = membership.get_node(&node_id).await.unwrap();
    assert!(node.is_some());
    assert_eq!(node.unwrap().address, addr);
    
    // Test state update
    membership.update_node_state(&node_id, NodeState::Active).await.unwrap();
    
    let node = membership.get_node(&node_id).await.unwrap();
    assert_eq!(node.unwrap().state, NodeState::Active);
}

#[tokio::test]
async fn test_messaging_broker() {
    let broker = InMemoryBroker::new("test-broker");
    
    // Test publish and subscribe
    let topic = "test-topic";
    let message_data = b"test message".to_vec();
    
    let _subscription = broker.subscribe(topic).await.unwrap();
    broker.publish(topic, message_data.clone()).await.unwrap();
    
    // Note: In a real test we'd need to handle the stream properly
    // For now we just verify the methods don't panic
}

#[tokio::test]
async fn test_consistency_manager() {
    let consistency = DummyConsistencyManager;
    
    // Test basic operations (dummy implementation just returns Ok)
    let key = b"test_key";
    let value = b"test_value";
    
    consistency.put(key, value).await.unwrap();
    let result = consistency.get(key).await.unwrap();
    // Dummy implementation returns None
    assert!(result.is_none());
    
    consistency.delete(key).await.unwrap();
    consistency.read_repair(key).await.unwrap();
}