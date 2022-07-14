use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        let mut prefix: Vec<i32> = vec![1; nums.len()];
        let mut suffix: Vec<i32> = vec![1; nums.len()];

        for i in 0..nums.len() {
            let left = nums.len() - i - 1;

            if i == 0 {
                prefix[0] = nums[i];
                suffix[left] = nums[left];
                continue;
            }

            prefix[i] = nums[i] * prefix[i - 1];
            suffix[left] = nums[left] * suffix[left + 1];
        }

        for i in 0..nums.len() {
            if i == 0 {
                res.push(suffix[1]);
                continue;
            }

            if i == nums.len() - 1 {
                res.push(prefix[i - 1]);
                continue;
            }

            res.push(prefix[i - 1] * suffix[i + 1]);
        }

        res
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
    fn product_except_self_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![3, 2], expected: vec![2, 3] },
            TestCase { nums: vec![1, 2, 3, 4], expected: vec![24, 12, 8, 6] },
            TestCase { nums: vec![-1, 1, 0, -3, 3], expected: vec![0, 0, 9, 0, 0] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::product_except_self(tc.nums));
        }
    }
}
