use coretex::{
    storage::{StorageEngine, InMemoryEngine, WriteOperation},
    membership::{MembershipManager, InMemoryMembership, NodeState},
    messaging::{MessageBroker, memory::InMemoryBroker},
    distribution::{DistributionStrategy, ConsistentHashRing, DistributionNode},
};
use std::collections::HashMap;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Coretex Distributed Storage System Demo");
    println!("============================================");
    
    // 1. Storage Engine Demo
    println!("\n📦 Storage Engine Demo:");
    let storage = InMemoryEngine::new("demo-storage");
    
    // Store some key-value pairs
    storage.put(b"user:1", b"Alice").await?;
    storage.put(b"user:2", b"Bob").await?;
    storage.put(b"config:timeout", b"30s").await?;
    
    // Retrieve values
    if let Some(value) = storage.get(b"user:1").await? {
        println!("  user:1 = {}", String::from_utf8_lossy(&value));
    }
    
    // Batch operations
    let batch_ops = vec![
        WriteOperation::Put { 
            key: Bytes::from("batch:1"), 
            value: Bytes::from("value1") 
        },
        WriteOperation::Put { 
            key: Bytes::from("batch:2"), 
            value: Bytes::from("value2") 
        },
    ];
    storage.batch_write(batch_ops).await?;
    println!("  ✅ Batch operations completed");
    
    // 2. Membership Management Demo
    println!("\n🌐 Membership Management Demo:");
    let membership = InMemoryMembership::new();
    
    // Register nodes
    let node1_id = membership.register_node(
        "127.0.0.1:8001".parse()?,
        HashMap::from([("role".to_string(), "primary".to_string())])
    ).await?;
    
    let node2_id = membership.register_node(
        "127.0.0.1:8002".parse()?,
        HashMap::from([("role".to_string(), "replica".to_string())])
    ).await?;
    
    println!("  📍 Registered node 1: {}", node1_id);
    println!("  📍 Registered node 2: {}", node2_id);
    
    // Update node states
    membership.update_node_state(&node1_id, NodeState::Active).await?;
    membership.update_node_state(&node2_id, NodeState::Active).await?;
    
    let nodes = membership.get_nodes().await?;
    println!("  📊 Active nodes: {}", nodes.len());
    
    // 3. Messaging System Demo
    println!("\n📨 Messaging System Demo:");
    let broker = InMemoryBroker::new("demo-broker");
    
    // Subscribe to a topic (in a real scenario, you'd handle the stream)
    let _subscription = broker.subscribe("cluster-events").await?;
    
    // Publish messages
    broker.publish("cluster-events", b"Node joined".to_vec()).await?;
    broker.publish("cluster-events", b"Configuration updated".to_vec()).await?;
    println!("  ✅ Published messages to cluster-events topic");
    
    // 4. Distribution Strategy Demo
    println!("\n🔄 Distribution Strategy Demo:");
    let mut hash_ring = ConsistentHashRing::new();
    
    // Add nodes to the hash ring
    let dist_node1 = DistributionNode {
        id: node1_id.clone(),
        weight: 100,
    };
    let dist_node2 = DistributionNode {
        id: node2_id.clone(),
        weight: 100,
    };
    
    hash_ring.add_node(dist_node1).await;
    hash_ring.add_node(dist_node2).await;
    
    // Test key distribution
    let test_keys = [b"key1", b"key2", b"key3", b"key4"];
    for key in test_keys.iter() {
        if let Some(primary) = hash_ring.get_primary(*key).await {
            println!("  🔑 {} -> Primary: {}", 
                String::from_utf8_lossy(*key), 
                primary.id
            );
        }
    }
    
    // Get replicas for a key
    let replicas = hash_ring.get_replicas(b"important_data", 2).await;
    println!("  📋 Replicas for 'important_data': {} nodes", replicas.len());
    
    println!("\n✨ Demo completed successfully!");
    println!("Coretex provides a pure Rust implementation of distributed storage");
    println!("with no dependencies on external services like etcd, NIXL, or jetstream.");
    
    Ok(())
}