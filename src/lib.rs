/// 题目存档
pub mod solutions {
    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
    pub mod p0003_length_of_longest_substring;
}

/// 常用函数
pub mod utils;

#[cfg(test)]
pub mod tests {
    use crate::utils::quick_sort;

    pub mod p0001_two_sum;
    pub mod p0002_add_two_numbers;
    pub mod p0003_length_of_longest_substring;

    #[derive(PartialEq, Debug)]
    struct lee {
        x: i32,
        y: i32
    }

    impl lee {
        pub fn new(x: i32, y: i32) -> Self {
            lee { x, y }
        }
    }

    #[test]
    pub fn test_my_thought() {
        let mut v = vec![3, 1, 4, 2, 9];
        quick_sort(&mut v, &|x, y| x < y);
        assert_eq!(v, vec![1, 2, 3, 4, 9]);
        quick_sort(&mut v, &|x, y| x > y);
        assert_eq!(v, vec![9, 4, 3, 2, 1]);
        let mut v = vec![lee::new(1, 2), lee::new(2, 1)];
        quick_sort(&mut v, &|x: &lee, y: &lee| x.x < y.x);
        assert_eq!(v, vec![lee::new(1, 2), lee::new(2, 1)]);
    }
}