use crate::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        for i in 0..nums.len() {
            while nums[i] != i as i32 + 1 {
                let num = nums[i];
                if num < 1 || num as usize >= nums.len() {
                    break;
                }

                let ix = num as usize - 1;
                if nums[ix] == num {
                    break;
                }

                let buf = nums[ix];

                nums[ix] = num;
                nums[i] = buf;
            }
        }

        for i in 0..nums.len() {
            let num = nums[i];
            let expected = i as i32 + 1;

            if num != expected {
                return expected;
            }
        }

        return nums.len() as i32 + 1;
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
    fn first_missing_positive_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![-1, 4, 2, 1, 9, 10], expected: 3 },
            TestCase { nums: vec![1, 2, 0], expected: 3 },
            TestCase { nums: vec![3, 4, -1, 1], expected: 2 },
            TestCase { nums: vec![7, 8, 9, 11, 12], expected: 1 },
            TestCase { nums: vec![3, 2, 1, 7, 8, 9, 11, 12], expected: 4 },
            TestCase { nums: vec![1, 2, 3, 4, 5], expected: 6 },
            TestCase { nums: vec![1, 1, 1], expected: 2 },
            TestCase { nums: vec![9, 8, 1, 6, 5, 4, 3, 2, 2, 10, 11, 21, -7, 14], expected: 7 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::first_missing_positive(tc.nums));
        }
    }
}
