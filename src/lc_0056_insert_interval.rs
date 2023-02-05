use crate::Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let len = intervals.len();
        let start = new_interval[0];
        let mut was_added = false;
        for i in 0..len {
            let interval = &intervals[i];
            if start <= interval[0] {
                intervals.insert(i, new_interval.clone());
                was_added = true;
                break;
            }
        }
        if !was_added {
            intervals.push(new_interval);
        }

        intervals
            .into_iter()
            .fold(Vec::<Vec<i32>>::new(), |mut acc, curr_interval| {
                let intervals_to_add = match acc.pop() {
                    Some(last_interval) => {
                        if overlap(&last_interval, &curr_interval) {
                            let start = last_interval[0].min(curr_interval[0]);
                            let end = last_interval[1].max(curr_interval[1]);
                            vec![vec![start, end]]
                        } else {
                            vec![last_interval, curr_interval]
                        }
                    }
                    _ => vec![curr_interval],
                };

                for interval_to_add in intervals_to_add {
                    acc.push(interval_to_add);
                }

                acc
            })
    }
}

fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    if a[0] < b[0] {
        return a[1] >= b[0];
    } else if b[0] < a[0] {
        return b[1] >= a[0];
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
        assert_eq!(ans, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        );
        assert_eq!(ans, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn overlap_1_test() {
        let ans = overlap(&vec![1, 3], &vec![4, 6]);
        assert_eq!(ans, false);
    }

    #[test]
    fn overlap_2_test() {
        let ans = overlap(&vec![1, 4], &vec![4, 6]);
        assert_eq!(ans, true);
    }

    #[test]
    fn overlap_3_test() {
        let ans = overlap(&vec![1, 5], &vec![4, 6]);
        assert_eq!(ans, true);
    }
}
