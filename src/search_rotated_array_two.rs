use crate::Solution;

impl Solution {
    pub fn search_rotated_array_two(nums: Vec<i32>, target: i32) -> bool {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;

        while l <= r {
            let pos = l + (r - l) / 2;

            if nums[l] == target || nums[r] == target || nums[pos] == target {
                return true;
            }

            if l == r {
                break;
            }

            if nums[l] == nums[r] {
                l += 1;
                r -= 1;
                continue;
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

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        expected: bool,
    }

    #[test]
    fn search_rotated_two_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![2, 5, 6, 0, 0, 1, 2],
                target: 0,
                expected: true,
            },
            TestCase {
                nums: vec![2, 5, 6, 0, 0, 1, 2],
                target: 3,
                expected: false,
            },
            TestCase {
                nums: vec![2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                target: 1,
                expected: true,
            },
            TestCase {
                nums: vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                target: 3,
                expected: false,
            },
            TestCase {
                nums: vec![2],
                target: 7,
                expected: false,
            },
            TestCase {
                nums: vec![7],
                target: 7,
                expected: true,
            },
            TestCase {
                nums: vec![1, 2, 1],
                target: 1,
                expected: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::search_rotated_array_two(tc.nums, tc.target));
        }
    }
}
