use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut mem: HashMap<i32, i32> = HashMap::new();

        return Self::coin_change_rec(&coins, amount, &mut mem);
    }

    fn coin_change_rec(coins: &Vec<i32>, amount: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        if mem.get(&amount).is_some() {
            return *mem.get(&amount).unwrap();
        }

        if amount < 0 {
            return -1;
        }

        if amount == 0 {
            return 0;
        }

        if coins.len() == 1 {
            let diff = amount % coins[0];
            if diff != 0 {
                return -1;
            }

            return amount / coins[0];
        }

        let mut min = -1;
        for i in 0..coins.len() {
            let coin = coins[i];

            let val = Self::coin_change_rec(coins, amount - coin, mem);
            if val == -1 {
                continue;
            }

            let val = val + 1;

            if min == -1 || val < min {
                min = val;
            }
        }

        mem.insert(amount, min);

        return min;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        coins: Vec<i32>,
        amount: i32,
        expected: i32,
    }

    #[test]
    fn coin_change_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                coins: vec![1, 2, 5],
                amount: 11,
                expected: 3,
            },
            TestCase {
                coins: vec![2],
                amount: 1,
                expected: -1,
            },
            TestCase {
                coins: vec![2, 4],
                amount: 7,
                expected: -1,
            },
            TestCase {
                coins: vec![2],
                amount: 8,
                expected: 4,
            },
            TestCase {
                coins: vec![1],
                amount: 0,
                expected: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::coin_change(tc.coins, tc.amount));
        }
    }
}
