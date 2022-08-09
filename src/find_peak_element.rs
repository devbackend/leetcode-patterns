use crate::Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            if l == 0 && r == 1 || l == nums.len() - 2 && r == nums.len() - 1 {
                if nums[l] > nums[r] {
                    return l as i32;
                }

                return r as i32;
            }

            let pos = l + (r - l) / 2;

            if nums[pos - 1] < nums[pos] && nums[pos] > nums[pos + 1] {
                return pos as i32;
            }

            if nums[pos] < nums[pos - 1] {
                r = pos - 1;
            } else {
                l = pos + 1;
            }
        }

        return l as i32;
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
    fn find_peak_element_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2, 1, 3, 1], expected: 0 },
            TestCase { nums: vec![3, 4, 3, 2, 1], expected: 1 },
            TestCase { nums: vec![1, 2, 3, 1], expected: 2 },
            TestCase { nums: vec![1, 2, 3, 4], expected: 3 },
            TestCase { nums: vec![1, 2, 1, 3, 5, 6, 4], expected: 5 },
            TestCase { nums: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1], expected: 12 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_peak_element(tc.nums));
        }
    }
}
