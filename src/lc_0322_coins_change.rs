use crate::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let cc = CoinChange::new(&coins);
        cc.min_coins(amount)
    }
}

struct CoinChange {
    coins: Vec<i32>,
}

impl CoinChange {
    pub fn new(coins: &[i32]) -> Self {
        Self {
            coins: coins.into(),
        }
    }

    fn min_coins(&self, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut change = vec![i32::MAX; (1 + amount) as usize];
        change[0] = 0;

        for i in 1..=amount {
            for coin in 0..self.coins.len() {
                if self.coins[coin] <= i {
                    let cur_count = change[(i - self.coins[coin]) as usize];
                    if cur_count != i32::MAX {
                        change[i as usize] = change[i as usize].min(cur_count + 1);
                    }
                }
            }
        }
        if change[amount as usize] == i32::MAX {
            -1
        } else {
            change[amount as usize]
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
