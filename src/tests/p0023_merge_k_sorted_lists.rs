use crate::solutions::p0023_merge_k_sorted_lists::{merge_k_lists, ListNode};

fn list(values: Vec<i32>) -> Option<Box<ListNode>> {
    ListNode::from_vec(values)
}

#[test]
#[ignore = "等待完成 p0023 题解后启用"]
fn test_p0023_typical_case() {
    let result = merge_k_lists(vec![
        list(vec![1, 4, 5]),
        list(vec![1, 3, 4]),
        list(vec![2, 6]),
    ]);
    assert_eq!(ListNode::to_vec(&result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
}

#[test]
#[ignore = "等待完成 p0023 题解后启用"]
fn test_p0023_empty_input() {
    assert_eq!(ListNode::to_vec(&merge_k_lists(vec![])), Vec::<i32>::new());
}

#[test]
#[ignore = "等待完成 p0023 题解后启用"]
fn test_p0023_contains_empty_lists() {
    let result = merge_k_lists(vec![None, list(vec![]), list(vec![1]), None]);
    assert_eq!(ListNode::to_vec(&result), vec![1]);
}

#[test]
#[ignore = "等待完成 p0023 题解后启用"]
fn test_p0023_duplicate_values() {
    let result = merge_k_lists(vec![list(vec![2, 2]), list(vec![2]), list(vec![2, 2, 2])]);
    assert_eq!(ListNode::to_vec(&result), vec![2, 2, 2, 2, 2, 2]);
}

#[test]
#[ignore = "等待完成 p0023 题解后启用"]
fn test_p0023_negative_values() {
    let result = merge_k_lists(vec![list(vec![-10, -5, 0]), list(vec![-6, -2, 3])]);
    assert_eq!(ListNode::to_vec(&result), vec![-10, -6, -5, -2, 0, 3]);
}
