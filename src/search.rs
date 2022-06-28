use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let pos = (l + (r - l) / 2) as usize;

            if nums[pos] == target {
                return pos as i32;
            }

            if nums[pos] > target {
                r = pos as i32 - 1;
            } else {
                l = pos as i32 + 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        expected: i32,
    }

    #[test]
    fn search_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![-1, 0, 3, 5, 9, 12], target: 9, expected: 4 },
            TestCase { nums: vec![-1, 0, 3, 5, 9, 12], target: 2, expected: -1 },
            TestCase { nums: vec![-1, 0, 3, 5, 9, 12], target: -1, expected: 0 },
            TestCase { nums: vec![-1, 0, 3, 5, 9, 12], target: 12, expected: 5 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::search(tc.nums, tc.target));
        }
    }
}
