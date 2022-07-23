use crate::Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();

        Self::letter_case_permutation_rec(&chars, 0)
    }

    fn letter_case_permutation_rec(chars: &Vec<char>, ix: usize) -> Vec<String> {
        if ix == chars.len() {
            return vec![];
        }

        let ch = chars[ix];
        let mut current = vec![ch.to_string()];
        if !ch.is_numeric() {
            if ch.is_lowercase() {
                current.push(ch.to_uppercase().to_string())
            } else {
                current.push(ch.to_lowercase().to_string())
            }
        }

        if ix == chars.len() - 1 {
            return current;
        }

        let switched = Self::letter_case_permutation_rec(chars, ix + 1);
        if switched.len() == 0 {
            return current;
        }

        let mut res: Vec<String> = vec![];

        for i in 0..current.len() {
            for k in 0..switched.len() {
                let mut str = current[i].clone();
                str.push_str(switched[k].as_str());

                res.push(str)
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        s: String,
        expected: Vec<String>,
    }

    #[test]
    fn letter_case_permutation_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                s: "a1b2".to_string(),
                expected: vec!["a1b2".to_string(), "a1B2".to_string(), "A1b2".to_string(), "A1B2".to_string()],
            },
            TestCase {
                s: "3z4".to_string(),
                expected: vec!["3z4".to_string(), "3Z4".to_string()],
            },
            TestCase {
                s: "abc".to_string(),
                expected: vec!["abc".to_string(), "abC".to_string(), "aBc".to_string(), "aBC".to_string(), "Abc".to_string(), "AbC".to_string(), "ABc".to_string(), "ABC".to_string()],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::letter_case_permutation(tc.s));
        }
    }
}
