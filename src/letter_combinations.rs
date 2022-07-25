use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        let letters: HashMap<char, Vec<String>> = HashMap::from([
            ('2', vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            ('3', vec!["d".to_string(), "e".to_string(), "f".to_string()]),
            ('4', vec!["g".to_string(), "h".to_string(), "i".to_string()]),
            ('5', vec!["j".to_string(), "k".to_string(), "l".to_string()]),
            ('6', vec!["m".to_string(), "n".to_string(), "o".to_string()]),
            ('7', vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()]),
            ('8', vec!["t".to_string(), "u".to_string(), "v".to_string()]),
            ('9', vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()]),
        ]);

        let digit = digits.chars().nth(0).unwrap();

        if digits.len() == 1 {
            return letters.get(&digit).unwrap().clone();
        }

        let mut res: Vec<String> = vec![];

        for letter in letters.get(&digit).unwrap() {
            for additional in Self::letter_combinations(digits.clone().as_str()[1..].to_string()) {
                let mut str_res = String::from(letter);
                str_res.push_str(additional.as_str());

                res.push(str_res);
            }
        }

        return res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        digits: String,
        expected: Vec<String>,
    }

    #[test]
    fn letter_combinations_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                digits: "23".to_string(),
                expected: vec!["ad".to_string(), "ae".to_string(), "af".to_string(), "bd".to_string(), "be".to_string(), "bf".to_string(), "cd".to_string(), "ce".to_string(), "cf".to_string()],
            },
            TestCase {
                digits: "".to_string(),
                expected: vec![],
            },
            TestCase {
                digits: "2".to_string(),
                expected: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::letter_combinations(tc.digits));
        }
    }
}
