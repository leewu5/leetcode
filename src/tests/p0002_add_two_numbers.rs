use crate::solutions::p0002_add_two_numbers::{add_two_numbers, ListNode};

#[test]
fn test_p0002() {
    assert_eq!(
        add_two_numbers(ListNode::from_vec(vec![2, 4, 3]), ListNode::from_vec(vec![5, 6, 4])), 
        ListNode::from_vec(vec![7, 0, 8])
    );
    assert_eq!(
        add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0])), 
        ListNode::from_vec(vec![0])
    );
}