pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let c: Vec<char> = s.chars().collect();
    let mut dp: Vec<Vec<bool>> = (0..n).map(|i| vec![false; n]).collect();
    for i in 0..n {
        dp[i][i] = true;
    }
    let mut max_len = 1;
    let mut index = 0;
    for len in 2..=n {
        for i in 0..(n-len+1) {
            let j = i + len - 1;
            if c[i] == c[j] && (len == 2 || dp[i+1][j-1]) {
                dp[i][j] = true;
                if max_len < len {
                    max_len = len;
                    index = i;
                }
            }
        }
    }
    return s[index..index+max_len].to_string();
}