use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return false;
        }

        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }

        if sum % 2 == 1 {
            return false;
        }

        let mut mem: HashMap<(usize, i32), bool> = HashMap::new();

        Self::has_target(&nums, 0, sum / 2, &mut mem)
    }

    fn has_target(nums: &Vec<i32>, ix: usize, target: i32, mem: &mut HashMap<(usize, i32), bool>) -> bool {
        let key = (ix, target);

        if mem.get(&key).is_some() {
            return *mem.get(&key).unwrap();
        }

        if target == 0 {
            return true;
        }

        if target < 0 {
            return false;
        }

        if ix == nums.len() - 1 {
            return nums[ix] == target;
        }

        let next = ix + 1;
        let mut res = false;

        for i in next..nums.len() {
            if Self::has_target(nums, i, target - nums[ix], mem) {
                res = true;
                break;
            }
        }

        mem.insert(key, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: bool,
    }

    #[test]
    fn can_partition_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![1, 5, 11, 5], expected: true },
            TestCase { nums: vec![1, 2, 3, 5], expected: false },
            TestCase { nums: vec![1; 199], expected: false },
            TestCase { nums: vec![2; 200], expected: true },
            TestCase {
                nums: vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 99, 97],
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::can_partition(tc.nums));
        }
    }
}
