use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];

        for i in 1..nums.len() {
            res = res ^ nums[i];
        }

        res
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
    fn single_number_test() {
        let test_cases = vec![
            TestCase { nums: vec![2, 2, 1], expected: 1 },
            TestCase { nums: vec![4, 1, 2, 1, 2], expected: 4 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::single_number(tc.nums));
        }
    }
}
