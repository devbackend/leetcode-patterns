use crate::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut items = Vec::new();

        Self::combination_sum_three_rec(k, n, 1, &mut res, &mut items);

        return res;
    }

    fn combination_sum_three_rec(k: i32, n: i32, from: i32, res: &mut Vec<Vec<i32>>, items: &mut Vec<i32>) {
        if k == 0 && n == 0 {
            res.push(items.clone());
            return;
        }

        if k <= 0 || n <= 0 {
            return;
        }

        for i in from..10 {
            items.push(i);
            Self::combination_sum_three_rec(k - 1, n - i, i + 1, res, items);
            items.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        k: i32,
        n: i32,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn combination_sum3_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { k: 2, n: 11, expected: vec![vec![2, 9], vec![3, 8], vec![4, 7], vec![5, 6]] },
            TestCase { k: 3, n: 7, expected: vec![vec![1, 2, 4]] },
            TestCase { k: 3, n: 9, expected: vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]] },
            TestCase { k: 4, n: 1, expected: vec![] },
            TestCase { k: 9, n: 60, expected: vec![] },
        ];

        for tc in test_cases {
            let mut actual = Solution::combination_sum3(tc.k, tc.n);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
