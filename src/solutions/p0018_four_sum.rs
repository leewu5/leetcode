use std::collections::VecDeque;

struct Status {
    // 初始指向数组头
    cursor: usize,
    // 当前状态下最大可选数字的编号
    // tail: usize,
    // 当前和
    sum: i64,
    // 当前选中的数字
    stack: VecDeque<i32>,
}

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    // 数组长度小于4直接返回空数组
    if nums.len() < 4 {
        return vec![];
    }
    let mut r = vec![];
    // 排序
    nums.sort_unstable();
    let mut status = Status {
        cursor: 0,
        // tail: nums.len(),
        sum: 0,
        stack: VecDeque::new(),
    };
    solve(&mut status, &nums, &mut r, target);
    return r;
}

fn solve(status: &mut Status, nums: &Vec<i32>, r: &mut Vec<Vec<i32>>, target: i32) {
    if status.stack.len() == 4 {
        if status.sum.eq(&target.into()) {
            let mut t = vec![];
            status.stack.iter().for_each(|n| t.push(*n));
            r.push(t);
        }
        return;
    }
    let mut i = status.cursor;
    while i < nums.len() {
        status.sum += (nums[i] as i64);
        status.cursor = i + 1;
        status.stack.push_back(nums[i]);
        solve(status, nums, r, target);
        status.stack.pop_back();
        status.sum -= (nums[i] as i64);
        i = find_next_ne_num(i, nums);
    }
}

fn find_next_ne_num(i: usize, nums: &Vec<i32>) -> usize {
    let mut j = i + 1;
    while j < nums.len() && nums[j] == nums[i] {
        j += 1;
    }
    return j;
}
