use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut baskets: HashSet<i32> = HashSet::new();

        let mut start = 0;
        let mut max = 0;
        for end in 0..fruits.len() {
            let fruit = fruits[end];

            if baskets.len() < 2 || baskets.contains(&fruit) {
                baskets.insert(fruit);
                max = i32::max(max, (end - start) as i32 + 1);
            } else {
                let prev = fruits[end - 1];

                for i in (0..=end - 2).rev() {
                    if fruits[i] != prev {
                        let current_fruit = fruits[i];

                        start = i + 1;
                        baskets.remove(&current_fruit);
                        break;
                    }
                }

                baskets.insert(fruit);
                max = i32::max(max, (end - start) as i32 + 1);
            }
        }

        return max;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        fruits: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn total_fruit_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { fruits: vec![1, 2, 1], expected: 3 },
            TestCase { fruits: vec![0, 1, 2, 2, 1], expected: 4 },
            TestCase { fruits: vec![1, 2, 3, 2, 3, 2], expected: 5 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::total_fruit(tc.fruits));
        }
    }
}
