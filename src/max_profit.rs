use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];

        for i in 1..prices.len() {
            let profit = prices[i] - min_price;

            if profit > max_profit {
                max_profit = profit;
            }

            if prices[i] < min_price {
                min_price = prices[i];
            }
        }

        return max_profit;
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
    fn max_profit_test() {
        let test_cases = vec![
            TestCase { prices: vec![7, 1, 5, 3, 6, 4], expected: 5 },
            TestCase { prices: vec![7, 6, 4, 3, 1], expected: 0 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::max_profit(tc.prices));
        }
    }
}
