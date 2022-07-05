use crate::Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut first: usize = nums[0] as usize;
        let mut second: usize = nums[0] as usize;

        loop {
            first = nums[first] as usize;
            second = nums[nums[second] as usize] as usize;

            if first == second {
                break;
            }
        }

        first = nums[0] as usize;

        while first != second {
            first = nums[first] as usize;
            second = nums[second] as usize;
        }

        first as i32
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
    fn find_duplicate_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![1, 3, 4, 2, 2], expected: 2 },
            TestCase { nums: vec![3, 1, 3, 4, 2], expected: 3 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_duplicate(tc.nums));
        }
    }
}
