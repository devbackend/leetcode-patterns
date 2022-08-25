use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut repeats: HashMap<char, usize> = HashMap::new();

        let chars: Vec<char> = s.chars().collect();

        let mut max = 0;

        let mut start: usize = 0;
        for end in 0..chars.len() {
            let char = chars[end];

            if repeats.get(&char).is_some() {
                start = usize::max(start, *repeats.get(&char).unwrap() + 1);
            }

            repeats.insert(char, end);
            max = i32::max(max, (end - start + 1) as i32);
        }

        return max;
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
    fn length_of_longest_substring_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                s: "abba".to_string(),
                expected: 2,
            },
            TestCase{
                s: "abcabcbb".to_string(),
                expected: 3,
            },
            TestCase{
                s: "bbbbb".to_string(),
                expected: 1,
            },
            TestCase{
                s: "pwwkews".to_string(),
                expected: 4,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::length_of_longest_substring(tc.s));
        }
    }
}
