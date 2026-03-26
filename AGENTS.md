# Saddle 项目说明

## 项目概况

- 项目名称：saddle
- 技术栈：Rust, tokio, ratatui, rusqlite, wasmtime
- 包管理器：cargo
- 简介：Harness-native CLI with local memory and plugin system

## 代码规范

### 命名约定
- 模块名：snake_case（如 `feature_manager`）
- 类型名：CamelCase（如 `FeatureManager`）
- 枚举成员：snake_case
- 常量：SCREAMING_SNAKE_CASE

### 错误处理
- 使用 `anyhow::Result<T>` 作为主要错误类型
- 库代码避免使用 `unwrap()`，使用 `?` 运算符传播错误
- 关键错误使用自定义错误枚举（如 `SaddleError`）

### 类型使用
- 充分利用 Rust 类型系统
- 优先使用 `Result` 而非 `Option` 处理可失败操作
- trait 对象（`Box<dyn Trait>`）仅在必要时使用

### 注释要求
- 公共 API 必须有 `///` 文档注释
- 复杂逻辑添加 `//` 行注释说明

## 目录结构

| 目录 | 用途 |
|------|------|
| `src/` | 源代码 |
| `src/harness/` | Harness 核心：功能清单、进度、交接 |
| `src/llm/` | LLM 适配层 |
| `src/memory/` | SQLite 记忆存储 |
| `src/plugins/` | WASM 插件运行时 |
| `src/tui/` | TUI 界面 |
| `src/utils/` | 工具模块 |
| `.crush/` | Crush 工作目录 |
| `.crush/skill/` | 技能包目录 |
| `.crush/agents/` | 子代理配置目录 |
| `.crush/init/` | 初始化脚本目录 |
| `harness/` | 项目管理文件（features.json, progress.md, handoff.md） |
| `plugins/` | 编译后的 .wasm 插件 |

## 常用命令

```bash
cargo run          # 运行
cargo test         # 测试
cargo build --release  # 发布构建
cargo clippy       # 代码检查
cargo fmt          # 格式化
```

## 常见坑点

1. **异步运行时**：统一使用 tokio，避免混用其他运行时
2. **WASM 插件**：注意 WASM 与宿主之间的类型传递（使用 wit-bindgen）
3. **SQLite**：写入时使用事务，避免数据损坏
4. **功能清单**：`harness/features.json` 必须保持 JSON 格式有效，写入时原子操作
5. **TUI 渲染**：避免在异步任务中阻塞 UI 线程

## Harness 工作流

1. 读取 `harness/features.json` 获取功能清单
2. 按依赖顺序实现功能，更新 `status` 为 `completed`
3. 更新 `harness/progress.md` 记录进度
4. 重要节点生成 `harness/handoff.md` 交接报告

## 依赖关系

功能清单中的依赖关系必须遵守：
- `infra-001` 是所有其他功能的基础
- `harness-*` 模块之间有依赖链
- `memory-001` 是 `memory-002` 的基础
- `plugin-001` 是 `plugin-002`、`plugin-003` 的基础
