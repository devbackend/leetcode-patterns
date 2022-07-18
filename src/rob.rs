use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut mem: HashMap<usize, i32> = HashMap::new();

        Self::rob_rec(&mut nums, 0, &mut mem)
    }

    fn rob_rec(nums: &mut Vec<i32>, ix: usize, mem: &mut HashMap<usize, i32>) -> i32 {
        if mem.get(&ix).is_some() {
            return *mem.get(&ix).unwrap();
        }

        if ix >= nums.len() {
            return 0;
        }

        if ix == nums.len() - 1 {
            return nums[ix];
        }

        if ix == nums.len() - 2 {
            if nums[ix] > nums[ix + 1] {
                return nums[ix];
            }

            return nums[ix + 1];
        }

        let first = nums[ix] + Self::rob_rec(nums, ix + 2, mem);
        let second = Self::rob_rec(nums, ix + 1, mem);

        if first > second {
            mem.insert(ix, first);
            return first;
        }

        mem.insert(ix, second);
        return second;
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
    fn rob_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![1, 2, 3, 1], expected: 4 },
            TestCase { nums: vec![1, 5], expected: 5 },
            TestCase { nums: vec![2, 7, 9, 3, 1], expected: 12 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::rob(tc.nums));
        }
    }
}
