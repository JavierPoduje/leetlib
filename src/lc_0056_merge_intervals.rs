use crate::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        intervals.iter().fold(Vec::new(), |mut acc, curr| {
            if acc.is_empty() {
                acc.push(curr.clone());
            } else {
                let prev = acc.last().unwrap();
                let end = prev[1].max(curr[1]);
                if overlap(&prev, curr) {
                    acc.last_mut().unwrap()[1] = end;
                } else {
                    acc.push(curr.clone());
                }
            }

            acc
        })
    }
}

fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    a.first() == b.first() || a.last() >= b.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}
