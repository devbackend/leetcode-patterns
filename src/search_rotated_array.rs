use crate::Solution;

impl Solution {
    pub fn search_rotated_array(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        while l <= r {
            let pos = l + (r - l) / 2;

            if nums[pos] == target {
                return pos as i32;
            }

            if nums[l] <= nums[pos] {
                if nums[pos] < target || target < nums[l] {
                    l = pos + 1;
                } else {
                    r = pos - 1;
                }
            } else {
                if nums[pos] > target || target > nums[r] {
                    r = pos - 1;
                } else {
                    l = pos + 1;
                }
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
    fn search_rotated_two_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![8, 9, 10, 1, 2, 3, 4, 5, 6, 7],
                target: 1,
                expected: 3,
            },
            TestCase {
                nums: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                target: 8,
                expected: 8,
            },
            TestCase {
                nums: vec![4, 5, 6, 7, 0, 1, 2],
                target: 4,
                expected: 0,
            },
            TestCase {
                nums: vec![4, 5, 6, 7, 0, 1, 2],
                target: 3,
                expected: -1,
            },
            TestCase {
                nums: vec![8, 9, 10, 1, 2, 3, 4, 5, 6, 7],
                target: 11,
                expected: -1,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::search_rotated_array(tc.nums, tc.target));
        }
    }
}
