use std::{cmp::max, collections::HashSet};

pub fn length_of_longest_substring(s: String) -> i32 {
    let n = s.len();
    if n == 0 {return 0;}
    let mut set = HashSet::new();
    let mut ans = 1;
    let cs: Vec<char> = s.chars().collect();
    let mut  r = 0;
    for i in 0..n {
        if (i > 0) {
            set.remove(&cs[i-1]);
        }
        while r < n && !set.contains(&cs[r]) {
            set.insert(cs[r]);
            r = r + 1;
        }
        ans = max(ans, r - i);
    }
    ans as i32
}