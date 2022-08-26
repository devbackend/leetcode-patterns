use crate::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let current_sum = nums[i] + nums[l] + nums[r];

                if current_sum > 0 {
                    r -= 1;
                } else if current_sum < 0 {
                    l += 1;
                } else {
                    res.push(vec![nums[i], nums[l], nums[r]]);

                    l += 1;
                    while nums[l] == nums[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn three_sum_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![0, 1, 1], expected: vec![] },
            TestCase { nums: vec![0, 0, 0], expected: vec![vec![0, 0, 0]] },
            TestCase { nums: vec![-1, 0, 1, 2, -1, -4], expected: vec![vec![-1, -1, 2], vec![-1, 0, 1]] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::three_sum(tc.nums));
        }
    }
}
