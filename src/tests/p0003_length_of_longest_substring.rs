use crate::solutions::p0003_length_of_longest_substring::length_of_longest_substring;

#[test]
pub fn test_p0003 () {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("".to_string()), 0);
}