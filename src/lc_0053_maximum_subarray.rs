use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_subarray = i32::MIN;
        let mut curr_sum = 0;

        for num in nums {
            if curr_sum < 0 {
                curr_sum = 0;
            }
            curr_sum += num;
            max_subarray = max_subarray.max(curr_sum);
        }

        max_subarray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(ans, 6);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::max_sub_array(vec![1]);
        assert_eq!(ans, 1);
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::max_sub_array(vec![5, 4, -1, 7, 8]);
        assert_eq!(ans, 23);
    }
}
