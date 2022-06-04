use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        n * (n + 1) / 2 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn missing_number_test() {
        let test_cases = vec![
            TestCase { nums: vec![3, 0, 1], expected: 2 },
            TestCase { nums: vec![0, 1], expected: 2 },
            TestCase { nums: vec![9, 6, 4, 2, 3, 5, 7, 0, 1], expected: 8 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::missing_number(tc.nums));
        }
    }
}