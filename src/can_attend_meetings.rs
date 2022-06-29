use crate::Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        if intervals.len() < 2 {
            return true;
        }

        let mut intervals = intervals;

        intervals.sort_by(|first, second| first.start.cmp(&second.start));

        for i in 0..intervals.len() - 1  {
            if intervals[i].end > intervals[i + 1].start {
                return false
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::can_attend_meetings::Interval;
    use crate::Solution;

    struct TestCase {
        intervals: Vec<Interval>,
        expected: bool,
    }

    #[test]
    fn can_attend_meetings_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { intervals: vec![Interval::new(0, 30), Interval::new(5, 10), Interval::new(15, 20)], expected: false },
            TestCase { intervals: vec![Interval::new(5, 8), Interval::new(9, 15)], expected: true },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::can_attend_meetings(tc.intervals));
        }
    }
}

// Definition of Interval:
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}