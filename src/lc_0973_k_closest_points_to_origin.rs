use crate::Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut distance_by_point: Vec<(&Vec<i32>, f64)> = points
            .iter()
            .map(|point| {
                let x = i32::pow(point[0], 2);
                let y = i32::pow(point[1], 2);

                let distance = ((x + y) as f64).sqrt();

                (point, distance)
            })
            .collect();

        distance_by_point.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        distance_by_point
            .iter()
            .take(k as usize)
            .map(|(point, _)| point.clone().to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        assert_eq!(ans, vec![vec![-2, 2]]);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        assert_eq!(ans, vec![vec![3, 3], vec![-2, 4]]);
    }
}
