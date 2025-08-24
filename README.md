# Coretex

Coretex 是一个用 Rust 编写的高性能分布式存储系统，灵感来源于 Amazon Dynamo，架构灵活、可插拔，适合分布式 KV 存储、配置中心等场景。

## 特点

- **无第三方强依赖**：不依赖 ETCD、Nats、NIXL 等外部服务，核心功能全部自实现。
- **模块化设计**：存储引擎、成员管理、消息通信、一致性、分布策略等均为 trait，可灵活扩展和替换。
- **异步高性能**：基于 Rust async/await 和 tokio，充分发挥多核性能。
- **易于扩展**：支持内存、RocksDB、Sled 等多种存储后端，支持自定义一致性和分布策略。
- **适合学习与生产**：代码结构清晰，便于二次开发和分布式系统学习。

## 目录结构

```
coretex/
├── Cargo.toml
├── src/
│   ├── main.rs            # 程序入口
│   ├── lib.rs             # 核心库
│   ├── error.rs           # 错误定义
│   ├── storage/           # 存储引擎 trait 及实现
│   ├── membership/        # 节点成员管理
│   ├── messaging/         # 消息通信
│   ├── consistency/       # 一致性管理
│   ├── config/            # 配置系统
│   ├── distribution/      # 数据分布策略
│   ├── api/               # 客户端 API
│   └── utils/             # 工具模块
└── examples/              # 示例
```

## 快速开始

1. **编译项目**
   ```bash
   cargo build
   ```

2. **运行节点**
   ```bash
   cargo run -- ./config.toml
   ```

3. **示例配置文件**
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

4. **本地客户端调用**
   可直接通过 API 或 examples 目录下的用例进行测试。

## 主要模块说明

- `storage`：存储引擎接口与实现（如内存、RocksDB、Sled）
- `membership`：节点注册、状态变更、成员发现
- `messaging`：节点间消息通信（内存实现，后续可扩展 TCP、gRPC 等）
- `consistency`：一致性协议与冲突解决
- `distribution`：一致性哈希等分布策略
- `config`：配置加载与热更新
- `api`：客户端 API

## 贡献

欢迎提交 Issue 和 PR，或提出架构建议！

## License

MIT License

---

本项目由 [lipish](https://github.com/lipish) 维护。