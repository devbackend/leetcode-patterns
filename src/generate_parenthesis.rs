use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }

        let mut res: Vec<String> = Vec::new();
        let mut mem: Vec<String> = Vec::new();

        Self::generate_parenthesis_rec(n, 0, 0, &mut res, &mut mem);

        return res;
    }

    fn generate_parenthesis_rec(n: i32, opened: i32, closed: i32, res: &mut Vec<String>, mem: &mut Vec<String>) {
        if opened == closed && opened == n {
            res.push(mem.join(""));
            return;
        }

        if opened < n {
            mem.push("(".to_string());
            Self::generate_parenthesis_rec(n, opened + 1, closed, res, mem);
            mem.pop();
        }

        if closed < opened {
            mem.push(")".to_string());
            Self::generate_parenthesis_rec(n, opened, closed + 1, res, mem);
            mem.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        n: i32,
        expected: Vec<String>,
    }

    #[test]
    fn generate_parenthesis_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { n: 1, expected: vec!["()".to_string()] },
            TestCase { n: 3, expected: vec!["((()))".to_string(), "(()())".to_string(), "(())()".to_string(), "()(())".to_string(), "()()()".to_string()] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::generate_parenthesis(tc.n));
        }
    }
}
