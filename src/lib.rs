/// 题目存档
pub mod solutions {
    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
}

/// 常用函数
pub mod utils;

#[cfg(test)]
pub mod tests {
    use crate::solutions::p0002_add_two_numbers::ListNode;

    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;

    #[test]
    pub fn test_my_thought() {
        assert_eq!(ListNode::from_vec(vec![1, 2]), ListNode::from_vec(vec![1, 2]));
    }
}