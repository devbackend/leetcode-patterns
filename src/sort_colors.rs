use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;

        while i <= r {
            let num = nums[i];

            if num == 0 && i > l {
                let tmp = nums[l];
                nums[l] = 0;
                nums[i] = tmp;

                l += 1;
            } else if num == 2 && i < r {
                let tmp = nums[r];
                nums[r] = 2;
                nums[i] = tmp;

                r -= 1;
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn sort_colors_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1],
                expected: vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2],
            },
            TestCase {
                nums: vec![2, 0, 2, 1, 1, 0],
                expected: vec![0, 0, 1, 1, 2, 2],
            },
            TestCase {
                nums: vec![2, 0, 1],
                expected: vec![0, 1, 2],
            },
        ];

        for tc in test_cases {
            let mut nums = tc.nums;

            Solution::sort_colors(&mut nums);

            assert_eq!(tc.expected, nums);
        }
    }
}
