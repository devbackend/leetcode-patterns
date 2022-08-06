use crate::Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut items = Vec::new();

        candidates.sort();

        Self::combination_sum_two_rec(&candidates, target, 0, &mut res, &mut items);

        return res;
    }

    fn combination_sum_two_rec(candidates: &Vec<i32>, target: i32, ix: usize, res: &mut Vec<Vec<i32>>, items: &mut Vec<i32>) {
        if target == 0 {
            res.push(items.clone());
            return;
        }

        if target < 0 || ix == candidates.len() {
            return;
        }

        let mut prev = -1;
        for i in ix..candidates.len() {
            if candidates[i] == prev {
                continue;
            }

            prev = candidates[i];

            items.push(candidates[i]);
            Self::combination_sum_two_rec(candidates, target - candidates[i], i + 1, res, items);
            items.pop();
        }
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
    fn combination_sum2_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { candidates: vec![1; 99], target: 7, expected: vec![vec![1, 1, 1, 1, 1, 1, 1]] },
            TestCase { candidates: vec![2, 5, 2, 1, 2], target: 5, expected: vec![vec![1, 2, 2], vec![5]] },
            TestCase { candidates: vec![10, 1, 2, 7, 6, 1, 5], target: 8, expected: vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]] },
        ];

        for tc in test_cases {
            let mut actual = Solution::combination_sum2(tc.candidates, tc.target);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
