use crate::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut start = 0;
        let mut sum = 0;

        for end in 0..nums.len() {
            sum += nums[end];

            while sum >= target {
                min = i32::min(min, (end - start + 1) as i32);
                sum -= nums[start];
                start += 1;
            }
        }

        if min == i32::MAX {
            return 0;
        }

        return min;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        target: i32,
        nums: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn min_sub_array_len_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                target: 7,
                nums: vec![2, 3, 1, 2, 4, 3],
                expected: 2,
            },
            TestCase {
                target: 4,
                nums: vec![1, 4, 4],
                expected: 1,
            },
            TestCase {
                target: 11,
                nums: vec![1, 1, 1, 1, 1, 1, 1, 1],
                expected: 0,
            },
            TestCase {
                target: 5,
                nums: vec![1; 100000],
                expected: 5,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::min_sub_array_len(tc.target, tc.nums));
        }
    }
}
