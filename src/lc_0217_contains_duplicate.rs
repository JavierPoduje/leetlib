use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for num in nums {
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(ans, true)
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(ans, false)
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(ans, true)
    }
}
