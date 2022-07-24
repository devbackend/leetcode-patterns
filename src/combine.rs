use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::combine_rec(0, n, k)
    }

    fn combine_rec(from: i32, to: i32, k: i32) -> Vec<Vec<i32>> {
        let mut nums = vec![];

        for i in from..to {
            nums.push(vec![i + 1]);
        }

        if k == 1 {
            return nums;
        }

        let mut res = vec![];

        for i in 0..nums.len() {
            let combinations = Self::combine_rec(nums[i][0], to, k - 1);

            for j in 0..combinations.len() {
                let mut variant = nums[i].clone();
                let mut combination = combinations[j].clone();
                variant.append(&mut combination);

                res.push(variant);
            }
        }

        return res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        n: i32,
        k: i32,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn combine_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                n: 1,
                k: 1,
                expected: vec![vec![1]],
            },
            TestCase {
                n: 3,
                k: 1,
                expected: vec![vec![1], vec![2], vec![3]],
            },
            TestCase {
                n: 4,
                k: 2,
                expected: vec![
                    vec![2, 4],
                    vec![3, 4],
                    vec![2, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                ],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::combine(tc.n, tc.k);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
