use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut dict: Vec<Vec<char>> = Vec::new();
        let mut mem: HashMap<usize, bool> = HashMap::new();

        for x in word_dict {
            dict.push(x.chars().collect())
        }

        Self::word_break_rec(&chars, &dict, 0, &mut mem)
    }

    fn word_break_rec(s: &Vec<char>, word_dict: &Vec<Vec<char>>, ix: usize, mem: &mut HashMap<usize, bool>) -> bool {
        if mem.get(&ix).is_some() {
            return *mem.get(&ix).unwrap();
        }

        if ix == s.len() {
            return true;
        }

        let mut res = false;
        for word in word_dict {
            let end = ix + word.len();
            if end > s.len() {
                continue;
            }

            if &s[ix..end] != word {
                continue;
            }

            res = Self::word_break_rec(s, word_dict, end, mem);
            if res {
                break;
            }
        }

        mem.insert(ix, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        s: String,
        word_dict: Vec<String>,
        expected: bool,
    }

    #[test]
    fn word_break_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                s: "leetcode".to_string(),
                word_dict: vec![
                    "leet".to_string(),
                    "code".to_string(),
                ],
                expected: true,
            },
            TestCase {
                s: "applepenapple".to_string(),
                word_dict: vec![
                    "apple".to_string(),
                    "pen".to_string(),
                ],
                expected: true,
            },
            TestCase {
                s: "catsandog".to_string(),
                word_dict: vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string(),
                ],
                expected: false,
            },
            TestCase {
                s: "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string(),
                word_dict: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                    "f".to_string(),
                    "g".to_string(),
                    "h".to_string(),
                    "i".to_string(),
                    "j".to_string(),
                    "k".to_string(),
                    "l".to_string(),
                    "m".to_string(),
                    "n".to_string(),
                    "o".to_string(),
                    "p".to_string(),
                    "q".to_string(),
                    "r".to_string(),
                    "s".to_string(),
                    "t".to_string(),
                    "u".to_string(),
                    "v".to_string(),
                    "w".to_string(),
                    "x".to_string(),
                    "y".to_string(),
                ],
                expected: false,
            },
            TestCase {
                s: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
                word_dict: vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string(),
                    "aaaaa".to_string(),
                    "aaaaaa".to_string(),
                    "aaaaaaa".to_string(),
                    "aaaaaaaa".to_string(),
                    "aaaaaaaaa".to_string(),
                    "aaaaaaaaaa".to_string(),
                ],
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::word_break(tc.s, tc.word_dict));
        }
    }
}
