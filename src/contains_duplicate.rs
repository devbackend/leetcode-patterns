use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut exists: HashMap<i32, bool> = HashMap::new();

        if nums.len() < 2 {
            return false;
        }

        for num in nums {
            if exists.contains_key(&num) {
                return true;
            }

            exists.insert(num, true);
        }

        return false;
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
    fn contains_duplicate_test() {
        let test_cases = vec![
            TestCase { nums: vec![1, 2, 3, 1], expected: true },
            TestCase { nums: vec![1, 2, 3], expected: false },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::contains_duplicate(tc.nums));
        }
    }
}
