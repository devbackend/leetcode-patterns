use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut max_len = 0;

        let mut mem: HashMap<usize, (i32, i32)> = HashMap::new();

        for i in (0..nums.len()).rev() {
            let mut curr_max_len = 1;
            let mut curr_cnt = 1;

            for j in (i + 1)..nums.len() {
                if nums[j] <= nums[i] {
                    continue;
                }

                let val = mem.get(&j).unwrap();
                if val.0 + 1 > curr_max_len {
                    curr_max_len = val.0 + 1;
                    curr_cnt = val.1;
                } else if val.0 + 1 == curr_max_len {
                    curr_cnt += val.1
                }
            }

            if curr_max_len > max_len {
                max_len = curr_max_len;
                res = curr_cnt;
            } else if curr_max_len == max_len {
                res += curr_cnt;
            }

            mem.insert(i, (curr_max_len, curr_cnt));
        }

        return res;
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
    fn find_number_of_lis_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2, 2, 2, 2, 2], expected: 5 },
            TestCase { nums: vec![1, 3, 5, 4, 7], expected: 2 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_number_of_lis(tc.nums));
        }
    }
}
