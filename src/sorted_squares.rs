use crate::Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![nums[0] * nums[0]];
        }

        let mut nums = nums;

        let mut first = 0;
        let mut second = nums.len() - 1;

        let mut res = vec![0; nums.len()];;

        for i in (0..nums.len()).rev() {
            let first_sq = nums[first] * nums[first];
            let second_sq = nums[second] * nums[second];

            if first_sq > second_sq {
                res[i] = first_sq;
                first += 1;
            } else {
                res[i] = second_sq;
                second -= 1;
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
        expected: Vec<i32>,
    }

    #[test]
    fn sorted_squares_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![-4, -1, 0, 3, 10], expected: vec![0, 1, 9, 16, 100] },
            TestCase { nums: vec![-7, -3, 2, 3, 11], expected: vec![4, 9, 9, 49, 121] },
            TestCase { nums: vec![-7, -6, -5, -4, 0], expected: vec![0, 16, 25, 36, 49] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::sorted_squares(tc.nums));
        }
    }
}
