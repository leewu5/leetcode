use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        match map.get(&(target - nums[i])) {
            None => {},
            Some(j) => return vec![*j as i32, i as i32]
        }
        map.insert(nums[i], i);
    }
    return vec![];
}