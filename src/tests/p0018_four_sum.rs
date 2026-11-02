use crate::solutions::p0018_four_sum::four_sum;

#[test]
pub fn test_four_sum() {
    let one = vec![1,0,-1,0,-2,2];
    assert_eq!(four_sum(one, 0), vec![[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]);
    let two = vec![2,2,2,2,2];
    assert_eq!(four_sum(two, 8), vec![[2,2,2,2]]);
    let three = vec![-3,-2,-1,0,0,1,2,3];
    assert_eq!(four_sum(three, 0), vec![[-3,-2,2,3],[-3,-1,1,3],[-3,0,0,3],[-3,0,1,2],[-2,-1,0,3],[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]);
    let four = vec![1,-2,-5,-4,-3,3,3,5];
    assert_eq!(four_sum(four, -11), vec![[-5,-4,-3,1]]);
    let five = vec![1,0,-1,0,-2,2];
    assert_eq!(four_sum(five, 0), vec![[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]])
}