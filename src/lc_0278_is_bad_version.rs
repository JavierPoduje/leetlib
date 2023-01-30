#[allow(dead_code)]
struct Solution {
    bad_version: i32,
}

impl Solution {
    #[allow(dead_code)]
    pub fn new(bad_version: i32) -> Self {
        Self { bad_version }
    }

    #[allow(dead_code)]
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if self.isBadVersion(1) {
            return 1;
        }

        let mut good = 1i64;
        let mut bad = n as i64;
        loop {
            let mid = (good + bad) / 2;
            if self.isBadVersion(mid as i32) {
                bad = mid;
            } else {
                good = mid;
            }

            if good + 1 == bad {
                return bad as i32;
            }
        }
    }

    #[allow(non_snake_case, dead_code)]
    fn isBadVersion(&self, n: i32) -> bool {
        self.bad_version <= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let fbv = Solution::new(4);
        assert_eq!(fbv.first_bad_version(5), 4);
    }

    #[test]
    fn ex2_test() {
        let fbv = Solution::new(1);
        assert_eq!(fbv.first_bad_version(1), 1);
    }

    #[test]
    fn ex3_test() {
        let fbv = Solution::new(1702766719);
        assert_eq!(fbv.first_bad_version(2126753390), 1702766719);
    }
}
