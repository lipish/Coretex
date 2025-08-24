# Coretex

Coretex is a high-performance distributed storage system written in Rust, inspired by Amazon Dynamo. It features a modular, pluggable architecture suitable for distributed KV storage, configuration centers, and more.

## Features

- **No Strong Third-party Dependencies**: No reliance on ETCD, Nats, NIXL, or other external services. All core functions are self-implemented.
- **Modular Design**: Storage engine, membership, messaging, consistency, and distribution strategies are all defined as traits, making them easy to extend or replace.
- **Async High Performance**: Built on Rust async/await and tokio, fully leveraging multicore performance.
- **Easy to Extend**: Supports in-memory, RocksDB, Sled, and other storage backends. Custom consistency and distribution strategies are supported.
- **Great for Learning and Production**: Clear code structure, suitable for secondary development and distributed systems education.

## Directory Structure

```
coretex/
├── Cargo.toml
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Core library
│   ├── error.rs           # Error definitions
│   ├── storage/           # Storage engine traits & implementations
│   ├── membership/        # Node membership management
│   ├── messaging/         # Messaging layer
│   ├── consistency/       # Consistency management
│   ├── config/            # Configuration system
│   ├── distribution/      # Data distribution strategies
│   ├── api/               # Client API
│   └── utils/             # Utility modules
└── examples/              # Usage examples
```

## Quick Start

### Basic Usage

```bash
# Run the basic demo
cargo run --example basic_demo

# Run with custom config
cargo run -- config.toml

# Run tests
cargo test
```

### API Example

```rust
use coretex::storage::{StorageEngine, InMemoryEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a storage engine
    let storage = InMemoryEngine::new("my-store");
    
    // Store data
    storage.put(b"key1", b"value1").await?;
    
    // Retrieve data
    if let Some(value) = storage.get(b"key1").await? {
        println!("Retrieved: {}", String::from_utf8_lossy(&value));
    }
    
    Ok(())
}
```

1. **Build the project**
   ```bash
   cargo build
   ```

2. **Run a node**
   ```bash
   cargo run -- ./config.toml
   ```

3. **Example configuration file**
   ```toml
   [node]
   bind_address = "127.0.0.1:9000"
   data_dir = "./data"
   seed_nodes = ["127.0.0.1:9001"]

   [storage]
   engine = "memory"

   [replication]
   factor = 3
   read_quorum = 2
   write_quorum = 2

   [consistency]
   mode = "Eventual"
   vector_clock_enabled = true
   ```

4. **Local client usage**
   You can test via the API or the examples in the `examples` directory.

## Main Modules

- `storage`: Storage engine interfaces and implementations (e.g., in-memory, RocksDB, Sled)
- `membership`: Node registration, state changes, and membership discovery
- `messaging`: Inter-node messaging (in-memory implementation, extensible to TCP/gRPC, etc.)
- `consistency`: Consistency protocols and conflict resolution
- `distribution`: Consistent hashing and other distribution strategies
- `config`: Configuration loading and hot-reloading
- `api`: Client API

## Contributing

Issues and PRs are welcome! Suggestions for architecture are also appreciated.

## License

MIT License

---

Maintained by [lipish](https://github.com/lipish).