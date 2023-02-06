use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;

        let chars = s.chars().collect::<Vec<char>>();
        let mut seen: HashSet<char> = HashSet::new();
        let mut longest = 0;

        while right < chars.len() {
            while seen.contains(&chars[right]) && left < right {
                seen.remove(&chars[left]);
                left += 1;
            }
            seen.insert(chars[right]);
            right += 1;
            longest = longest.max(seen.len() as i32);
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(ans, 3);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(ans, 1);
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(ans, 3);
    }
}
