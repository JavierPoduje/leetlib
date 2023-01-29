pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;

    let mut mid = (high + low) / 2;

    while nums[mid] != target {
        if low >= high {
            return -1;
        }

        if nums[mid] > target {
            high = mid.saturating_sub(1);
        }

        if nums[mid] < target {
            low = mid + 1;
        }

        mid = (high + low) / 2;
    }

    mid as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(search(vec![], 5), -1);
    }

    #[test]
    fn ex4_test() {
        assert_eq!(search(vec![5], -5), -1);
    }

    #[test]
    fn ex5_test() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 13), -1);
    }
}
