use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn max_profit_two(prices: Vec<i32>) -> i32 {
        let mut mem: HashMap<(usize, Option<i32>), i32> = HashMap::new();

        Self::max_profit_rec(&prices, 0, None, &mut mem)
    }

    fn max_profit_rec(prices: &Vec<i32>, ix: usize, buy_price: Option<i32>, mem: &mut HashMap<(usize, Option<i32>), i32>) -> i32 {
        let key = (ix, buy_price);
        if mem.get(&key).is_some() {
            return *mem.get(&key).unwrap();
        }

        let is_bought = buy_price.is_some();

        if ix >= prices.len() {
            return 0;
        }

        if ix == prices.len() - 1 {
            if is_bought {
                return i32::max(prices[ix] - buy_price.unwrap(), 0);
            }

            return 0;
        }

        let res: i32;

        if !is_bought {
            res = i32::max(Self::max_profit_rec(prices, ix + 1, Some(prices[ix]), mem), Self::max_profit_rec(prices, ix + 1, None, mem));
        } else {
            let first = (prices[ix] - buy_price.unwrap()) + Self::max_profit_rec(prices, ix + 2, None, mem);
            let second = Self::max_profit_rec(prices, ix + 1, buy_price, mem);

            res = i32::max(first, second);
        }

        mem.insert(key, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        prices: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn max_profit_two_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                prices: vec![1, 2, 3, 0, 2],
                expected: 3,
            },
            TestCase {
                prices: vec![1],
                expected: 0,
            },
            TestCase {
                prices: vec![1, 2],
                expected: 1,
            },
            TestCase {
                prices: vec![2, 1],
                expected: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::max_profit_two(tc.prices));
        }
    }
}
