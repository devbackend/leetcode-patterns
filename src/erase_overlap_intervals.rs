use crate::Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort();

        let mut res = 0;

        let mut last_end = intervals[0][1];

        for i in 1..intervals.len() {
            let start = intervals[i][0];
            let end = intervals[i][1];

            if start >= last_end {
                last_end = end;
                continue;
            }

            res += 1;
            last_end = i32::min(last_end, end);
        }

        return res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        intervals: Vec<Vec<i32>>,
        expected: i32,
    }

    #[test]
    fn erase_overlap_intervals_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                intervals: vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]],
                expected: 1,
            },
            TestCase {
                intervals: vec![vec![1, 2], vec![1, 2], vec![1, 2]],
                expected: 2,
            },
            TestCase {
                intervals: vec![vec![1, 2], vec![2, 3]],
                expected: 0,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::erase_overlap_intervals(tc.intervals));
        }
    }
}
