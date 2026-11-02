use crate::solutions::p0005_longest_palindrome::longest_palindrome;

#[test]
pub fn test_p0005() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
}
