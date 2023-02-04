use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elemt = 0;
        let mut counter = 0;
        for num in nums {
            if counter == 0 {
                elemt = num;
                counter = 1;
            } else if elemt != num {
                counter -= 1;
            } else {
                counter += 1;
            }
        }
        elemt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
