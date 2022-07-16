use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut mem: HashMap<String, Vec<Vec<String>>> = HashMap::new();

        return Self::partition_rec(s, &mut mem);
    }

    fn partition_rec(s: String, mem: &mut HashMap<String, Vec<Vec<String>>>) -> Vec<Vec<String>> {
        if s.len() == 1 {
            return vec![vec![s]];
        }

        if mem.get(&s).is_some() {
            return mem.get(&s).unwrap().to_vec();
        }

        let mut res = Vec::new();

        if s == s.clone().chars().rev().collect::<String>() {
            res.push(vec![s.clone()]);
        }

        for r in 1..s.len() {
            let first_sub_str = s[0..r].to_string();
            if first_sub_str != first_sub_str.clone().chars().rev().collect::<String>() {
                continue;
            }

            let second_sub_str = s[r..].to_string();

            for items in Self::partition_rec(second_sub_str, mem) {
                let mut palindromes = vec![first_sub_str.clone()];
                let mut items = items;

                palindromes.append(&mut items);

                res.push(palindromes);
            }
        }

        mem.insert(s, res.clone());

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        str: String,
        expected: Vec<Vec<String>>,
    }

    #[test]
    fn partition_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { str: "aaa".to_string(), expected: vec![vec!["aaa".to_string()], vec!["a".to_string(), "aa".to_string()], vec!["a".to_string(), "a".to_string(), "a".to_string()], vec!["aa".to_string(), "a".to_string()]] },
            TestCase { str: "a".to_string(), expected: vec![vec!["a".to_string()]] },
            TestCase { str: "aab".to_string(), expected: vec![vec!["a".to_string(), "a".to_string(), "b".to_string()], vec!["aa".to_string(), "b".to_string()]] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::partition(tc.str));
        }
    }
}
