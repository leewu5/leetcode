use std::collections::HashSet;

use crate::solutions::p0380_randomized_set::RandomizedSet;

#[test]
pub fn test_p0380() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    assert_eq!(HashSet::from([1, 2]).contains(&obj.get_random()), true);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.get_random(), 2);
}

#[test]
pub fn test_p0380_1() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(0), true);
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(0), true);
    assert_eq!(HashSet::from([1, 2]).contains(&obj.get_random()), true);
    assert_eq!(obj.insert(2), true);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.get_random(), 2);
}