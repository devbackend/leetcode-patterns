use crate::Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }

        let chars: Vec<char> = s.chars().collect();

        let mut res = 0;
        for i in 0..chars.len() {
            res += Self::palindromes_count(&chars, i, i);
            res += Self::palindromes_count(&chars, i, i + 1);
        }

        return res
    }

    fn palindromes_count(chars: &Vec<char>, left: usize, right: usize) -> i32 {
        let mut res = 0;

        let mut left = left as i32;
        let mut right = right as i32;

        while left >= 0 && (right as usize) < chars.len() && chars[left as usize] == chars[right as usize] {
            left -= 1;
            right += 1;

            res += 1;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        s: String,
        expected: i32,
    }

    #[test]
    fn count_substrings_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{s: "abc".to_string(), expected: 3},
            TestCase{s: "aaa".to_string(), expected: 6},
            TestCase{s: "a".to_string(), expected: 1},
            TestCase{s: "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string(), expected: 500500},
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::count_substrings(tc.s));
        }
    }
}
