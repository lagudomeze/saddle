---
name: doc-writer
description: 编写技术文档，包含 API 文档、用户手册格式规范
---

# 技术文档编写技能

## 文档类型

### 1. API 文档（src/lib.rs 及模块）
```rust
//! 模块简短描述
//!
//! 详细说明，包括使用示例。
//!
//! # Example
//!
//! ```
//! use saddle::{FeatureManager, Feature};
//! let manager = FeatureManager::new()?;
//! ```

/// 类型或函数描述
///
/// # Arguments
///
/// * `name` - 参数说明
///
/// # Returns
///
/// 返回值说明
///
/// # Errors
///
/// 可能的错误及原因
pub fn function() {}
```

### 2. README.md
```markdown
# 项目名称

简短描述（1-2句话）

## 特性

- 特性1
- 特性2

## 快速开始

```bash
命令示例
```

## 文档目录

- [安装](installation.md)
- [使用](usage.md)
```

### 3. 用户手册
每章结构：
```markdown
## 章节标题

### 概述
介绍本章内容

### 前提条件
列出需要了解的内容

### 操作步骤
1. 步骤1
2. 步骤2

### 示例
```bash
示例命令
```

### 注意事项
重要提示和警告
```

## 格式化规范

- Markdown 标题层级：H1 > H2 > H3 > H4
- 代码块标注语言：`rust`, `bash`, `json`, `toml`
- 链接使用相对路径
- 图片使用 `docs/` 目录
