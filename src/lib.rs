/// 题目存档
pub mod solutions {
    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
    pub mod p0003_length_of_longest_substring;
    pub mod p0005_longest_palindrome;
    pub mod p0380_randomized_set;
    pub mod p0018_four_sum;
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
    pub mod p0380_randomized_set;
    pub mod p0018_four_sum;

    #[test]
    pub fn test_my_thought() {
        
    }
}