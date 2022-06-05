use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp;

        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for i in 1..nums.len() {
            current_sum = cmp::max(current_sum + nums[i], nums[i]);

            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }

        max_sum
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
    fn max_sub_array_test() {
        let test_cases = vec![
            TestCase { nums: vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], expected: 6 },
            TestCase { nums: vec![1], expected: 1 },
            TestCase { nums: vec![5, 4, -1, 7, 8], expected: 23 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::max_sub_array(tc.nums));
        }
    }
}
