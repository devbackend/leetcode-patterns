use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        if nums.len() == 0 {
            return 0;
        }

        let mut mem: HashMap<i32, bool> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];

            if mem.get(&num).is_some() {
                continue;
            }

            mem.insert(num, true);
        }

        let mut max = 1;
        for (num, _) in &mem {
            let prev = num - 1;
            if mem.get(&prev).is_some() {
                continue;
            }

            let mut diff = num + 1;
            while mem.get(&diff).is_some() {
                diff += 1;
            }

            max = i32::max(max, diff - num);
        }

        return max;
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
    fn longest_consecutive_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![100, 4, 200, 1, 3, 2], expected: 4 },
            TestCase { nums: vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], expected: 9 },
            TestCase { nums: vec![2, 2, 1, 3, 4, 5], expected: 5 },
            TestCase { nums: vec![1, 3, 5], expected: 1 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::longest_consecutive(tc.nums));
        }
    }
}
