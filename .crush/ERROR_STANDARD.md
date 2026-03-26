# Saddle Error 处理规范

## 概述

Saddle 项目使用统一的 `SaddleError` 枚举处理所有错误类型，基于 `exn` 库提供精确的调用栈信息。

## 错误定义模板

```rust
use derive_more::{Display, Error};
use crate::SaddleResult;

#[derive(Debug, Display, Error)]
pub enum SaddleError {
    #[display("IO error: {_0}")]
    Io(std::io::Error),
    #[display("Parse error: {_0}")]
    Parse(#[error(not(source))] String),
    #[display("Plugin error: {_0}")]
    Plugin(#[error(not(source))] String),
    #[display("Config error: {_0}")]
    Config(#[error(not(source))] String),
    #[display("Logging error: {_0}")]
    Logging(#[error(not(source))] String),
    #[display("Feature error: {_0}")]
    Feature(#[error(not(source))] String),
    #[display("Progress error: {_0}")]
    Progress(#[error(not(source))] String),
    #[display("Handoff error: {_0}")]
    Handoff(#[error(not(source))] String),
    #[display("TUI error: {_0}")]
    Tui(#[error(not(source))] String),
    #[display("LLM error: {_0}")]
    Llm(#[error(not(source))] String),
    #[display("LLM adapter error: {_0}")]
    LlmAdapter(#[error(not(source))] String),
    #[display("Memory error: {_0}")]
    Memory(#[error(not(source))] String),
    #[display("Init error: {_0}")]
    Init(#[error(not(source))] String),
    #[display("Other error: {_0}")]
    Other(#[error(not(source))] String),
}
```

## 统一错误类型 (SaddleError)

所有模块应使用 `SaddleError` 枚举的变体，禁止定义独立的 Error 结构体。

| 变体 | 用途 | 示例 |
|------|------|------|
| `Io` | 文件/网络 IO 操作 | 文件读写失败 |
| `Parse` | JSON/TOML/配置文件解析 | 格式错误 |
| `Plugin` | WASM 插件相关 | 加载失败 |
| `Config` | 配置加载/保存 | 配置缺失或无效 |
| `Logging` | 日志系统 | 初始化失败 |
| `Feature` | 功能清单管理 | features.json 操作 |
| `Progress` | 进度跟踪 | progress.md 操作 |
| `Handoff` | 交接报告生成 | handoff.md 操作 |
| `Tui` | 终端界面 | 渲染/输入错误 |
| `Llm` | LLM 客户端 | API 调用失败 |
| `LlmAdapter` | LLM 适配器 | 模型不支持 |
| `Memory` | 记忆存储 | SQLite 操作 |
| `Init` | 初始化流程 | 启动配置错误 |
| `Other` | 其他错误 | 未分类错误 |

## Result 类型别名

```rust
pub type SaddleResult<T> = exn::Result<T, SaddleError>;
```

## 使用示例

### 1. 基本错误转换

```rust
use crate::SaddleResult;

fn read_config() -> SaddleResult<Settings> {
    let content = std::fs::read_to_string("config.toml")
        .map_err(|e| crate::SaddleError::Io(e))?;
    // ...
}
```

### 2. 使用 exn 的 `or_raise`

```rust
use exn::{Result, ResultExt};

fn load_features() -> Result<Vec<Feature>, crate::SaddleError> {
    let content = std::fs::read_to_string("features.json")
        .or_raise(|| crate::SaddleError::Feature(
            "Failed to read features.json".into()
        ))?;
    // ...
}
```

### 3. 自定义错误消息

```rust
use crate::SaddleError;

fn validate_feature(feature: &Feature) -> SaddleResult<()> {
    if feature.id.is_empty() {
        return Err(SaddleError::Feature("Feature ID cannot be empty".into()));
    }
    Ok(())
}
```

### 4. 嵌套错误 (包装 IO 错误)

```rust
use crate::SaddleError;

fn open_database(path: &str) -> SaddleResult<Connection> {
    Connection::open(path)
        .map_err(|e| SaddleError::Io(e))
}
```

## 禁止事项

1. **禁止使用 `anyhow`** - 丢失调用栈信息
2. **禁止使用 `thiserror`** - 同样丢失调用栈
3. **禁止定义独立 Error 结构体** - 如 `struct ConfigError { message: String }`
4. **禁止返回裸 `Result`** - 应使用 `SaddleResult<T>`

## 文件结构

- 主错误定义：`src/utils.rs`
- 统一导出：`src/lib.rs` 的 `error` 模块
- 使用：`use crate::SaddleError;` 或 `use crate::SaddleResult;`

## 错误传播模式

```rust
// 好：使用 map_err 包装
std::fs::read_to_string(path)
    .map_err(|e| SaddleError::Io(e))?

// 好：使用 or_raise 添加上下文
fs::write(path, content)
    .or_raise(|| SaddleError::Config(format!("Failed to write to {:?}", path)))?

// 好：返回带消息的错误
Err(SaddleError::Feature("Invalid feature ID".into()))
```
