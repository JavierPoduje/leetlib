use crate::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        rec(nums.clone(), &mut permutations, Vec::new());
        permutations
    }
}

fn rec(nums: Vec<i32>, permutations: &mut Vec<Vec<i32>>, curr: Vec<i32>) {
    if nums.is_empty() {
        permutations.push(curr);
        return;
    }

    let nums_clone = nums.clone();
    for num in nums.clone() {
        let mut new_curr = curr.clone();
        new_curr.push(num);
        let clone = nums_clone
            .clone()
            .into_iter()
            .filter(|x| *x != num)
            .collect();

        rec(clone, permutations, new_curr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
