use crate::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut ways = vec![i32::MAX; (amount + 1) as usize];
        ways[0] = 0;

        for coin in coins {
            for denom in 0..=(amount as usize) {
                if coin <= denom as i32 && ways[denom - coin as usize] != i32::MAX {
                    ways[denom] = ways[denom].min(ways[denom - coin as usize] + 1);
                }
            }
        }

        match ways[amount as usize] {
            i32::MAX => -1,
            last => last,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::coin_change(vec![1, 2, 5], 11);
        assert_eq!(ans, 3);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::coin_change(vec![2], 3);
        assert_eq!(ans, -1);
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::coin_change(vec![1], 0);
        assert_eq!(ans, 0);
    }

    #[test]
    fn ex4_test() {
        let ans = Solution::coin_change(vec![186, 419, 83, 408], 6249);
        assert_eq!(ans, 20);
    }
}
