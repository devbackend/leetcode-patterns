use crate::Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }

        let mut nums = nums;

        let mut res: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            println!("{}", nums[i]);

            let pos = (nums[i].abs() as usize) - 1;

            if nums[pos] < 0 {
                res.push((pos as i32) + 1);
                continue;
            }

            nums[pos] *= -1;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn find_duplicates_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![4, 3, 2, 7, 8, 2, 3, 1], expected: vec![2, 3] },
            TestCase { nums: vec![1, 1, 2], expected: vec![1] },
            TestCase { nums: vec![1, 2, 3], expected: vec![] },
            TestCase { nums: vec![1], expected: vec![] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_duplicates(tc.nums));
        }
    }
}
