# Saddle

Harness-native CLI with local memory and plugin system.

## 项目目标

构建一个独立的 Rust CLI 工具，具备以下核心能力：

1. **Harness 原生约束**：双代理架构（初始化代理 + 编码代理）、功能清单驱动、进度文件和交接报告
2. **终端交互**：CLI 命令和可选的 TUI（基于 ratatui）
3. **本地记忆**：SQLite + 向量扩展实现长期记忆存储、检索和自动上下文注入
4. **插件系统**：基于 WebAssembly（wasmtime）的插件运行时
5. **自举能力**：最终能用自身驱动自身开发

## 快速开始

```bash
cargo run
```

## 项目结构

```
saddle/
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库入口
│   ├── cli.rs            # CLI参数解析
│   ├── harness/          # Harness核心
│   │   ├── feature_manager.rs
│   │   ├── progress_tracker.rs
│   │   └── handoff_generator.rs
│   ├── llm/              # LLM适配层
│   ├── memory/           # 记忆系统
│   ├── plugins/          # 插件系统
│   ├── tui/              # TUI界面
│   └── utils/            # 工具模块
├── harness/
│   ├── features.json     # 功能清单
│   ├── progress.md       # 进度跟踪
│   └── handoff.md        # 交接报告
└── plugins/              # WASM插件目录
```

## 常用命令

```bash
cargo run          # 运行
cargo test         # 测试
cargo build --release  # 发布构建
cargo clippy       # 代码检查
cargo fmt          # 格式化
```
