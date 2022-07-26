use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut max = nums[0];
        let mut diff: (i32, i32) = (1, 1);

        for i in 0..nums.len() {
            let num = nums[i];

            let new_min = num * diff.0;
            let new_max = num * diff.1;

            diff.0 = i32::min(new_max, i32::min(new_min, num));
            diff.1 = i32::max(new_max, i32::max(new_min, num));

            max = i32::max(max, diff.1);
        }

        return max;
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
    fn max_product_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![2, 3, -2, 4],
                expected: 6,
            },
            TestCase {
                nums: vec![1, 3, -5, 2, 4],
                expected: 8,
            },
            TestCase {
                nums: vec![-2, 0, -1],
                expected: 0,
            },
            TestCase {
                nums: vec![2, 8, 0, -1, -3],
                expected: 16,
            },
            TestCase {
                nums: vec![1; 9000],
                expected: 1,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::max_product(tc.nums));
        }
    }
}
