use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        if nums[nums.len() - 1] > nums[0] {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let pos = (left + (right - left) / 2) as usize;

            if nums[pos] > nums[pos + 1] {
                return nums[pos + 1];
            }

            if nums[left] > nums[pos] {
                right = pos - 1;
            } else {
                left = pos + 1;
            }
        }

        return 0;
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
    fn find_min_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![11, 13, 15, 17], expected: 11 },
            TestCase { nums: vec![3, 4, 5, 1, 2], expected: 1 },
            TestCase { nums: vec![4, 5, 6, 7, 0, 1, 2], expected: 0 },
            TestCase { nums: vec![9, 10, 1, 2, 3, 4, 5, 6, 7, 8], expected: 1 },
            TestCase { nums: vec![4, 5, 6, 7, 8, 9, 10, 2, 3], expected: 2 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_min(tc.nums));
        }
    }
}
