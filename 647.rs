use std::collections;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut sub: Vec<String> = Vec::new();
        sub = Self::getSubStrings(s);
        let mut count: i32 = 0;
        for s in sub {
            if Self::isPalindrome(s) {
                count += 1;
            }
        }
        return count;
    }

    fn getSubStrings(s: String) -> Vec<String> {
        let mut substrings = Vec::new();
        let n = s.len();

        for i in 0..n {
            for j in i + 1..=n {
                substrings.push(s[i..j].to_string());
            }
        }

        return substrings;
    }

    fn isPalindrome(s: String) -> bool {
        return s.chars().eq(s.chars().rev());
    }
}
