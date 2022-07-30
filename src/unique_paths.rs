use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut mem: HashMap<(i32, i32), i32> = HashMap::new();

        Self::unique_paths_rec(m, n, &mut mem)
    }

    fn unique_paths_rec(m: i32, n: i32, mem: &mut HashMap<(i32, i32), i32>) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        let key = (m, n);

        if mem.get(&key).is_some() {
            return *mem.get(&key).unwrap();
        }

        let res = Self::unique_paths_rec(m - 1, n, mem) + Self::unique_paths_rec(m, n - 1, mem);

        mem.insert(key, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        m: i32,
        n: i32,
        expected: i32,
    }

    #[test]
    fn unique_paths_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { m: 3, n: 7, expected: 28 },
            TestCase { m: 3, n: 2, expected: 3 },
            TestCase { m: 10, n: 10, expected: 48620 },
            TestCase { m: 40, n: 10, expected: 1677106640 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::unique_paths(tc.m, tc.n));
        }
    }
}
