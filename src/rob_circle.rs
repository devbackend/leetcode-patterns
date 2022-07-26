use crate::Solution;

impl Solution {
    pub fn rob_circle(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        return i32::max(
            Self::rob(nums[1..].to_vec()),
            Self::rob(nums[..nums.len() - 1].to_vec()),
        );
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
    fn rob_circle_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2, 3, 2], expected: 3 },
            TestCase { nums: vec![1, 2, 3, 1], expected: 4 },
            TestCase { nums: vec![1, 2, 3], expected: 3 },
            TestCase { nums: vec![1, 2, 3, 4, 5], expected: 8 },
            TestCase { nums: vec![5, 4, 10, 2, 20], expected: 30 },
            TestCase { nums: vec![1000; 100], expected: 50000 },
            TestCase { nums: vec![7], expected: 7 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::rob_circle(tc.nums));
        }
    }
}
