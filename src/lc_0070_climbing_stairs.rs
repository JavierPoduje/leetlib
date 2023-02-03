#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut prev = 1;
        let mut curr = 2;

        for _ in 2..n {
            let temp_curr = curr;
            curr += prev;
            prev = temp_curr;
        }

        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn ex2_test() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
