# LeetCode Rust 题解

这是一个使用 Rust 编写的 LeetCode 刷题记录仓库。每道题包含独立的题解代码和测试用例，便于按题号查阅、运行和补充。

## 项目结构

```text
.
├── Cargo.toml              # Cargo 项目配置与依赖
├── src/
│   ├── lib.rs              # 统一声明题解、工具和测试模块
│   ├── main.rs             # 二进制入口（用于临时实验）
│   ├── utils.rs            # 公共算法工具
│   ├── solutions/          # 题解实现
│   └── tests/               # 对应题解的测试
└── README.md
```

题解和测试文件采用相同的命名规则：

```text
p<题号>_<英文题名>.rs
```

例如，题号 `1` 的两数之和对应：

```text
src/solutions/p0001_two_sum.rs
src/tests/p0001_two_sum.rs
```

## 已完成题目

| 题号 | 题目 | 主要思路 |
| ---: | --- | --- |
| 0001 | Two Sum（两数之和） | 哈希表 |
| 0002 | Add Two Numbers（两数相加） | 链表模拟、进位 |
| 0003 | Longest Substring Without Repeating Characters（无重复字符的最长子串） | 滑动窗口、哈希集合 |
| 0005 | Longest Palindromic Substring（最长回文子串） | 动态规划 |
| 0018 | 4Sum（四数之和） | 排序、回溯、去重 |
| 0380 | Insert Delete GetRandom O(1) | 哈希表 + 动态数组 |

## 运行测试

在项目根目录执行：

```bash
# 运行全部测试
cargo test

# 显示测试输出
cargo test -- --nocapture

# 运行指定题目的测试
cargo test p0001
```

## 添加新题解

1. 在 `src/solutions/` 下新增题解文件，例如 `p0123_example.rs`。
2. 在 `src/tests/` 下新增同名测试文件。
3. 在 `src/lib.rs` 的 `solutions` 模块中声明题解模块。
4. 在 `src/lib.rs` 的 `tests` 模块中声明测试模块。
5. 使用 `cargo test` 验证实现。

题解模块建议只保留与题目相关的公开函数或类型，通用算法放入 `src/utils.rs`，并为每个题目补充覆盖典型情况和边界情况的测试。

## 依赖

项目当前使用 Rust 2021 edition，并通过 [`rand`](https://crates.io/crates/rand) 实现随机数操作。

## 目标

- 持续记录 Rust 版本的 LeetCode 解法。
- 为每道题保留可重复运行的测试。
- 在实践中整理常见的数据结构、算法和 Rust 编程技巧。
