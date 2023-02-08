use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut ans = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let times_by_num = nums.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let target = -1 * (nums[i] + nums[j]);

                if !times_by_num.contains_key(&target) {
                    continue;
                }

                let sub = if nums[i] == target && nums[j] == target {
                    2
                } else if nums[i] == target || nums[j] == target {
                    1
                } else {
                    0
                };

                let (smaller, middler, greater) = sort_logically(nums[i], nums[j], target);

                let key = format!("{}-{}-{}", smaller, middler, greater);
                if times_by_num[&target] - sub > 0 && !seen.contains(&key) {
                    seen.insert(key);
                    ans.push(vec![smaller, middler, greater]);
                }
            }
        }

        ans
    }
}

fn sort_logically(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
    let greater = a.max(b).max(c);
    let smaller = a.min(b).min(c);

    let middle = if a == greater && b == smaller || b == greater && a == smaller {
        c
    } else if b == greater && c == smaller || b == smaller && c == greater {
        a
    } else {
        b
    };
    (smaller, middle, greater)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(ans, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::three_sum(vec![0, 1, 1]);
        assert_eq!(ans, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::three_sum(vec![0, 0, 0]);
        assert_eq!(ans, vec![vec![0, 0, 0]]);
    }
}
