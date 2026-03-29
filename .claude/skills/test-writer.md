---
name: test-writer
description: 按 cargo test 规范编写单元测试和集成测试
---

# Rust 测试编写技能

## 测试组织

### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        assert_eq!(2 + 2, 4);
    }
}
```

### 集成测试
放在 `tests/` 目录，每个文件作为一个测试crate。

## 测试规范

### 命名
- 测试函数：`test_<被测函数>_<场景>`
- 描述性名称，清晰表达测试意图

### 断言
- 使用 `assert!`、`assert_eq!`、`assert_ne!`
- 优先使用 `assert_eq!` 而非 `assert!`（提供更好的错误信息）
- 考虑使用 `debug_assert!` 系列（仅调试时生效）

### 错误处理测试
```rust
#[test]
fn test_error_propagation() {
    let result = fallible_function();
    assert!(result.is_err());
}
```

### 边界条件
- 空值、零值、最大值
- 溢出/下溢情况
- 并发场景

## 输出格式

```rust
#[cfg(test)]
mod unit_tests {
    #[test]
    fn test_<feature>_<condition>_<expected>() {
        // Arrange
        let input = ...;

        // Act
        let result = ...;

        // Assert
        assert_eq!(result, expected);
    }
}
```
