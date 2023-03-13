use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let c = Combinator::new(&candidates, target);
        c.collect()
    }
}

struct Combinator {
    nums: Vec<i32>,
    target: i32,
    mult: Vec<i32>,
}

impl Combinator {
    pub fn new(candidates: &[i32], target: i32) -> Self {
        let mut nums = candidates.to_vec().clone();
        let len = nums.len();
        nums.sort();

        Self {
            nums,
            target,
            mult: vec![0; len],
        }
    }

    fn value(&self) -> i32 {
        self.nums
            .iter()
            .zip(self.mult.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }

    fn step(&mut self) -> bool {
        let mut idx = 0;
        while idx < self.nums.len() {
            self.mult[idx] += 1;
            if self.value() <= self.target {
                return true;
            }
            self.mult[idx] = 0;
            idx += 1;
        }

        false
    }
}

impl Iterator for Combinator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.step() {
            return None;
        }

        let mut loop_limit = 1000;
        while self.value() != self.target {
            if !self.step() {
                return None;
            }
            loop_limit -= 1;
            if loop_limit == 0 {
                panic!("possible infinite loop");
            }
        }

        let mut result = Vec::new();
        for (idx, val) in self.mult.iter().enumerate() {
            for _ in 0..*val {
                result.push(self.nums[idx]);
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn ex3_test() {
        let empty: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::combination_sum(vec![2], 1), empty);
    }
}
