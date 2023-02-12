use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut completed = vec![0; num_courses as usize];
        let mut map = HashMap::new();
        for p in prerequisites {
            map.entry(p[0]).or_insert(HashSet::new()).insert(p[1]);
            completed[p[0] as usize] += 1;
        }

        let mut zeroes = completed
            .iter()
            .enumerate()
            .filter(|x| *x.1 == 0)
            .map(|x| x.0 as i32)
            .collect::<Vec<i32>>();

        while let Some(zero) = zeroes.pop() {
            for (k, v) in map.iter_mut() {
                if v.remove(&zero) {
                    completed[*k as usize] -= 1;
                    if completed[*k as usize] == 0 {
                        zeroes.push(*k as i32);
                    }
                }
            }
        }

        completed.iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::can_finish(3, vec![vec![1, 0]]), true);
    }

    #[test]
    fn ex4_test() {
        assert_eq!(
            Solution::can_finish(
                4,
                vec![vec![2, 0], vec![1, 0], vec![3, 1], vec![3, 2], vec![1, 3]]
            ),
            false
        );
    }

    #[test]
    fn ex5_test() {
        assert_eq!(
            Solution::can_finish(
                20,
                vec![
                    vec![0, 10],
                    vec![3, 18],
                    vec![5, 5],
                    vec![6, 11],
                    vec![11, 14],
                    vec![13, 1],
                    vec![15, 1],
                    vec![17, 4]
                ]
            ),
            false
        );
    }

    #[test]
    fn ex6_test() {
        assert_eq!(
            Solution::can_finish(5, vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]),
            true
        );
    }

    #[test]
    fn ex7_test() {
        assert_eq!(
            Solution::can_finish(
                8,
                vec![
                    vec![1, 0],
                    vec![2, 6],
                    vec![1, 7],
                    vec![6, 4],
                    vec![7, 0],
                    vec![0, 5]
                ]
            ),
            true
        );
    }
}
