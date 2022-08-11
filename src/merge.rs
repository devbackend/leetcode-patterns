use crate::Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals;
        }

        let mut intervals = intervals;

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
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn merge_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                intervals: vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
                expected: vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            },
            TestCase {
                intervals: vec![vec![1, 4], vec![4, 5]],
                expected: vec![vec![1, 5]],
            },
            TestCase {
                intervals: vec![vec![3, 4], vec![2, 5], vec![1, 6], vec![0, 18]],
                expected: vec![vec![0, 18]],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::merge(tc.intervals));
        }
    }
}
