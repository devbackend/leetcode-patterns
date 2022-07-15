use crate::Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        res.push(0);

        for i in 1..n + 1 {
            let odd = i % 2;
            let cnt_key = (i / 2) as usize;

            res.push(res[cnt_key] + odd)
        }

        res
    }

    pub fn count_bits_with_builtin(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for i in 0..n + 1 {
            res.push(i.count_ones() as i32)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        num: i32,
        expected: Vec<i32>,
    }

    #[test]
    fn count_bits_test() {
        let test_cases = vec![
            TestCase { num: 0, expected: vec![0] },
            TestCase { num: 1, expected: vec![0, 1] },
            TestCase { num: 2, expected: vec![0, 1, 1] },
            TestCase { num: 5, expected: vec![0, 1, 1, 2, 1, 2] },
            TestCase { num: 10, expected: vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::count_bits(tc.num));
            assert_eq!(tc.expected, Solution::count_bits_with_builtin(tc.num));
        }
    }
}
