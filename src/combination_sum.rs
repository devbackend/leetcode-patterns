use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;

        Self::combination_sum_rec(&mut candidates, target, 0)
    }

    fn combination_sum_rec(candidates: &Vec<i32>, target: i32, ix: usize) -> Vec<Vec<i32>> {
        if target < 1 || ix == candidates.len() {
            return vec![]
        }

        let mut res = vec![];

        for i in ix..candidates.len() {
            if candidates[i] == target {
                res.push(vec![candidates[i]]);
                continue;
            }

            let combinations = Self::combination_sum_rec(candidates, target - candidates[i], i);

            for k in 0..combinations.len() {
                let mut variant = vec![candidates[i]];
                let mut combination = combinations[k].clone();

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
        candidates: Vec<i32>,
        target: i32,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn combination_sum_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                candidates: vec![2, 3, 6, 7],
                target: 7,
                expected: vec![vec![2, 2, 3], vec![7]],
            },
            TestCase {
                candidates: vec![2, 3, 5],
                target: 8,
                expected: vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            },
            TestCase {
                candidates: vec![2],
                target: 1,
                expected: vec![],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::combination_sum(tc.candidates, tc.target);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
