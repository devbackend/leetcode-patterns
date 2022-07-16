use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut mem: HashMap<(usize, i32), i32> = HashMap::new();

        return Self::find_target_sum_ways_rec(&nums, target, 0, &mut mem);
    }

    fn find_target_sum_ways_rec(nums: &Vec<i32>, target: i32, ix: usize, mem: &mut HashMap<(usize, i32), i32>) -> i32 {
        if ix == nums.len() - 1 {
            if nums[ix] == target || nums[ix] == -target {
                if nums[ix] == 0 {
                    return 2;
                }

                return 1;
            }

            return 0;
        }

        let key = (ix, target);

        if mem.get(&key).is_none() {
            let positive = Self::find_target_sum_ways_rec(nums, target - nums[ix], ix + 1, mem);
            let negative = Self::find_target_sum_ways_rec(nums, target + nums[ix], ix + 1, mem);

            mem.insert(key, positive + negative);
        }

        return *mem.get(&key).unwrap();
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
    fn find_target_sum_ways_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![1], target: 1, expected: 1 },
            TestCase { nums: vec![1], target: -1, expected: 1 },
            TestCase { nums: vec![1], target: 2, expected: 0 },
            TestCase { nums: vec![1, 1, 1, 1, 1], target: 3, expected: 5 },
            TestCase { nums: vec![1, 0], target: 1, expected: 2 },
            TestCase { nums: vec![1, 0, 1], target: 1, expected: 0 },
            TestCase { nums: vec![1, 0, 0, 0, 2], target: 3, expected: 8 },
            TestCase { nums: vec![5, 16, 45, 7, 20, 5, 32, 38, 43, 14, 29, 11, 2, 25, 36, 28, 27, 26, 45, 45], target: 17, expected: 6333 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_target_sum_ways(tc.nums, tc.target));
        }
    }
}
