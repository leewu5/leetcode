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

## 已完成题目与算法

| 题号 | 题目 | 算法 | 复杂度 |
| ---: | --- | --- | --- |
| 0001 | Two Sum（两数之和） | 哈希表记录已遍历元素，查找互补值 | 时间 `O(n)`，空间 `O(n)` |
| 0002 | Add Two Numbers（两数相加） | 同时遍历两条链表，逐位相加并维护进位 | 时间 `O(max(m, n))`，空间 `O(max(m, n))` |
| 0003 | Longest Substring Without Repeating Characters（无重复字符的最长子串） | 滑动窗口 + 哈希集合维护窗口内字符 | 时间 `O(n)`，空间 `O(n)` |
| 0005 | Longest Palindromic Substring（最长回文子串） | 动态规划判断区间是否为回文串 | 时间 `O(n²)`，空间 `O(n²)` |
| 0018 | 4Sum（四数之和） | 排序后固定前两个数，后两个数使用双指针，并跳过重复值 | 时间 `O(n³)`，空间 `O(log n)`（不计结果） |
| 0023 | Merge k Sorted Lists（合并 K 个升序链表） | 待完成；模板已提供顺序合并、分治合并和最小堆等思路提示 | 待补充 |
| 0380 | Insert Delete GetRandom O(1) | 哈希表保存下标，动态数组支持随机访问；删除时用末尾元素填补空位 | 插入/删除/随机均摊 `O(1)` |

每道题的实现文件中也会保留对应的算法说明、关键步骤和复杂度注释。随着题目数量增加，如果表格变得过长，可以将每道题的详细思路拆到 `docs/` 目录，README 只保留索引和摘要。

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
