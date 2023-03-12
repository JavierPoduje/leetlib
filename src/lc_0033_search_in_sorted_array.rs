use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            let val = nums[mid];

            if val == target {
                return mid as i32;
            }
            // handle left sorted portion
            else if nums[left] <= val {
                if val < target || target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            // handle right sorted portion
            else {
                if val > target || target > nums[right] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4)
    }
    #[test]
    fn ex2_test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1)
    }
    #[test]
    fn ex3_test() {
        assert_eq!(Solution::search(vec![1], 0), -1)
    }
}
