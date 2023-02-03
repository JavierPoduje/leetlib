#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        for ch in s.chars() {
            *hm.entry(ch).or_insert(0) += 1;
        }

        let mut max_len = 0;
        let mut has_odd = false;
        for val in hm.values() {
            if val % 2 == 0 {
                max_len += val;
            } else {
                max_len += val - 1;
                has_odd = true;
            }
        }

        max_len + if has_odd { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::longest_palindrome("ccc".to_string()), 3);
    }
}
