use crate::Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![vec![nums[0]]];
        }

        let mut nums = nums;

        let mut res = vec![];

        for _ in 0..nums.len() {
            let num = nums[0];
            nums = nums[1..].to_vec();

            let permutations = Self::permute(nums.clone());
            for k in 0..permutations.len() {
                let mut variant = permutations[k].clone();
                variant.push(num);

                res.push(variant);
            }

            nums.push(num);
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
    fn permute_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![1, 2, 3, 4],
                expected: vec![
                    vec![1, 2, 3, 4],
                    vec![1, 2, 4, 3],
                    vec![1, 3, 2, 4],
                    vec![1, 3, 4, 2],
                    vec![1, 4, 2, 3],
                    vec![1, 4, 3, 2],
                    vec![2, 1, 3, 4],
                    vec![2, 1, 4, 3],
                    vec![2, 3, 1, 4],
                    vec![2, 3, 4, 1],
                    vec![2, 4, 1, 3],
                    vec![2, 4, 3, 1],
                    vec![3, 1, 2, 4],
                    vec![3, 1, 4, 2],
                    vec![3, 2, 1, 4],
                    vec![3, 2, 4, 1],
                    vec![3, 4, 1, 2],
                    vec![3, 4, 2, 1],
                    vec![4, 1, 2, 3],
                    vec![4, 1, 3, 2],
                    vec![4, 2, 1, 3],
                    vec![4, 2, 3, 1],
                    vec![4, 3, 1, 2],
                    vec![4, 3, 2, 1],
                ],
            },
            TestCase {
                nums: vec![1, 2, 3],
                expected: vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]],
            },
            TestCase {
                nums: vec![0, 1],
                expected: vec![vec![0, 1], vec![1, 0]],
            },
            TestCase {
                nums: vec![2],
                expected: vec![vec![2]],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::permute(tc.nums);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
