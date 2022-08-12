use crate::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);

        if intervals.len() < 2 {
            return intervals;
        }

        intervals.sort();

        let mut res: Vec<Vec<i32>> = vec![intervals[0].clone()];
        for i in 1..intervals.len() {
            let ix = res.len() - 1;

            if res[ix][1] >= intervals[i][0] {
                res[ix] = vec![res[ix][0], i32::max(intervals[i][1], res[ix][1])];
                continue;
            }

            res.push(intervals[i].clone());
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        intervals: Vec<Vec<i32>>,
        new_interval: Vec<i32>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn insert_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                intervals: vec![vec![1, 3], vec![6, 9]],
                new_interval: vec![2, 5],
                expected: vec![vec![1, 5], vec![6, 9]],
            },
            TestCase {
                intervals: vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
                new_interval: vec![4, 8],
                expected: vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::insert(tc.intervals, tc.new_interval);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
