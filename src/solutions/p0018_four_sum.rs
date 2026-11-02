/// 返回数组中所有和为 `target` 的四元组。
///
/// 算法：先排序，然后固定前两个数，剩下两个数使用双指针查找。
/// 排序后可以在移动指针时跳过重复值，从而避免重复答案。
///
/// 时间复杂度：`O(n^3)`
/// 空间复杂度：`O(log n)`（排序所需的栈空间，不计返回值）
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 4 {
        return Vec::new();
    }

    nums.sort_unstable();
    let target = target as i64;
    let mut result = Vec::new();

    // 固定第一个数，最后三个位置留给 j、left 和 right。
    for i in 0..n - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // 最小和已经超过 target，后续 i 只会更大。
        let min_sum = nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64;
        if min_sum > target {
            break;
        }

        // 最大和仍小于 target，当前 i 不可能找到答案。
        let max_sum = nums[i] as i64 + nums[n - 1] as i64 + nums[n - 2] as i64 + nums[n - 3] as i64;
        if max_sum < target {
            continue;
        }

        // 固定第二个数，将问题转化为有序数组上的两数之和。
        for j in i + 1..n - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            let mut left = j + 1;
            let mut right = n - 1;
            while left < right {
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        // 跳过相同端点，避免产生重复四元组。
                        let left_value = nums[left];
                        let right_value = nums[right];
                        while left < right && nums[left] == left_value {
                            left += 1;
                        }
                        while left < right && nums[right] == right_value {
                            right -= 1;
                        }
                    }
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                }
            }
        }
    }

    result
}
