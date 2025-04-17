use crate::solutions::p0001_two_sum::two_sum;

#[test]
fn test_p0001() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    assert_eq!(vec![0, 1], result);
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = two_sum(nums, target);
    assert_eq!(vec![1, 2], result);
    let nums = vec![3, 3];
    let target = 6;
    let result = two_sum(nums, target);
    assert_eq!(vec![0, 1], result);
}