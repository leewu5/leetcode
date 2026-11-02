/// 题目存档
pub mod solutions {
    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
    pub mod p0003_length_of_longest_substring;
    pub mod p0005_longest_palindrome;
    pub mod p0018_four_sum;
    pub mod p0023_merge_k_sorted_lists;
    pub mod p0380_randomized_set;
}

/// 常用函数
pub mod utils;

#[cfg(test)]
pub mod tests {
    use crate::utils::quick_sort;

    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
    pub mod p0003_length_of_longest_substring;
    pub mod p0005_longest_palindrome;
    pub mod p0018_four_sum;
    pub mod p0023_merge_k_sorted_lists;
    pub mod p0380_randomized_set;

    #[test]
    pub fn test_my_thought() {}
}
