use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1
        }

        let mut mem: HashMap<usize, i32> = HashMap::new();
        let mut max = 1;

        mem.insert(nums.len() - 1, 1);

        Self::length_of_lis_rec(&nums, nums.len() - 2, &mut mem, &mut max);

        return max
    }

    fn length_of_lis_rec(nums: &Vec<i32>, ix: usize, mem: &mut HashMap<usize, i32>, max: &mut i32) {
        let mut curr_max = 1;

        for i in (ix + 1)..nums.len() {
            if nums[i] > nums[ix] {
                curr_max = i32::max(curr_max, 1 + *mem.get(&i).unwrap());
            }
        }

        mem.insert(ix, curr_max);

        *max = i32::max(*max, curr_max);

        if ix == 0 {
            return
        }

        Self::length_of_lis_rec(nums, ix - 1, mem, max)
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
    fn length_of_lis_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![4, 10, 4, 3, 8, 9], expected: 3 },
            TestCase { nums: vec![10, 9, 2, 5, 3, 7, 101, 18], expected: 4 },
            TestCase { nums: vec![0, 4, 1, 0, 3, 2, 3, 2, 1, 7], expected: 5 },
            TestCase { nums: vec![7, 7, 7, 7, 7, 7, 8], expected: 2 },
            TestCase { nums: (1..2499).collect(), expected: 2498 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::length_of_lis(tc.nums));
        }
    }
}
