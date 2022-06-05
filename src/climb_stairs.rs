use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut n1 = 1;
        let mut n2 = 2;

        for _ in 3..n + 1 {
            let buf = n2;

            n2 = n1 + n2;
            n1 = buf;
        }

        return n2;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        stairs: i32,
        expected: i32,
    }

    #[test]
    fn climb_stairs_test() {
        let test_cases = vec![
            TestCase { stairs: 1, expected: 1 },
            TestCase { stairs: 2, expected: 2 },
            TestCase { stairs: 3, expected: 3 },
            TestCase { stairs: 42, expected: 433_494_437 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::climb_stairs(tc.stairs));
        }
    }
}
