use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut mem: HashMap<i32, i32> = HashMap::new();

        Self::combination_sum_four_rec(&nums, target, &mut mem)
    }

    fn combination_sum_four_rec(nums: &Vec<i32>, target: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        if mem.get(&target).is_some() {
            return *mem.get(&target).unwrap()
        }

        if target < 0 {
            return 0;
        }

        if target == 0 {
            return 1;
        }

        let mut res = 0;

        for i in 0..nums.len() {
            res += Self::combination_sum_four_rec(nums, target - nums[i], mem);
        }

        mem.insert(target, res);

        return res;
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
    fn combination_sum4_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![3], target: 9, expected: 1 },
            TestCase { nums: vec![9], target: 3, expected: 0 },
            TestCase { nums: vec![1, 2, 3], target: 4, expected: 7 },
            TestCase { nums: (1..200).collect(), target: 16, expected: 32768 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::combination_sum4(tc.nums, tc.target));
        }
    }
}
