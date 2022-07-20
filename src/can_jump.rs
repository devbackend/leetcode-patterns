use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        return Self::can_jump_rec(&nums, nums.len() - 2, nums.len() - 1);
    }

    pub fn can_jump_rec(nums: &Vec<i32>, from: usize, to: usize) -> bool {
        let steps = nums[from];

        if steps >= (to - from) as i32 {
            if from == 0 {
                return true;
            }

            return Self::can_jump_rec(nums, from - 1, from);
        }

        if from == 0 {
            return false;
        }

        return Self::can_jump_rec(nums, from - 1, to);
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
    fn can_jump_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2], expected: true },
            TestCase { nums: vec![0, 2, 3], expected: false },
            TestCase { nums: vec![2, 3, 1, 1, 4], expected: true },
            TestCase { nums: vec![3, 2, 1, 0, 4], expected: false },
            TestCase { nums: vec![4, 2, 3, 1, 0, 0, 0, 0], expected: false },
            TestCase { nums: vec![4, 2, 3, 1, 0, 2, 0, 1], expected: true },
            TestCase { nums: vec![2, 4, 4, 3, 1, 1, 2, 1, 4, 1, 4, 1, 4, 3, 3, 3, 3, 4, 1, 4, 4, 4, 3, 3, 4, 3, 2, 4, 4, 2, 4, 4, 1, 4, 4, 2, 1, 4, 2, 4, 4, 4, 1, 3, 3, 2, 1, 3, 3, 2, 3, 3, 2, 4, 2, 4, 4, 4, 4, 1, 4, 3, 4, 1, 1, 2, 1, 1, 4, 3, 1, 1, 1, 2, 3, 4, 3, 2, 1, 2, 1, 3, 2, 1, 1, 4, 2, 3, 1, 4, 1, 2, 1, 4, 3, 3, 1, 2, 3, 3, 1, 1, 4, 3, 3, 3, 1, 2, 1, 3, 4, 1, 1, 3, 2, 3, 3, 4, 4, 3, 3, 2, 1, 2, 2, 3, 1, 4, 1, 3, 2, 4, 3, 3, 1, 3, 4, 3, 1, 4, 4, 3, 4, 1, 4, 1, 1, 2, 3, 3, 2, 2, 2, 1, 1, 3, 3, 4, 1, 2, 2, 3, 3, 2, 1, 3, 1, 1, 4, 4, 2, 1, 4, 1, 2, 1, 2, 2, 3, 4, 2, 4, 1, 2, 4, 4, 2, 4, 4, 2, 3, 3, 2, 1, 3, 1, 2, 1, 3, 3, 2, 1, 2, 2, 4, 3, 2, 2, 3, 3, 2, 2, 3, 3, 4, 3, 3, 1, 2, 1, 3, 1, 4, 3, 3, 1, 3, 3, 3, 2, 3, 4, 3, 4, 1, 2, 1, 1, 4, 2, 3, 3, 1, 4, 3, 1, 1, 4, 3, 4, 3, 3, 2, 2, 3, 2, 2, 2, 4, 2, 0, 0, 0, 0], expected: false },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::can_jump(tc.nums));
        }
    }
}
