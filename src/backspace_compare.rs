use crate::Solution;

impl Solution {
    pub fn backspace_compare(first_str: String, second_str: String) -> bool {
        let mut stack1: Vec<String> = Vec::new();
        let mut stack2: Vec<String> = Vec::new();

        let mut first = 0;
        let mut second = 0;

        while first < first_str.len() || second < second_str.len() {
            let first_ch = first_str.chars().nth(first);
            let second_ch = second_str.chars().nth(second);

            if first_ch.is_some() {
                let first_ch = first_ch.unwrap();

                if first_ch == '#' {
                    stack1.pop();
                } else {
                    stack1.push(first_ch.to_string());
                }
            }

            if second_ch.is_some() {
                let second_ch = second_ch.unwrap();

                if second_ch == '#' {
                    stack2.pop();
                } else {
                    stack2.push(second_ch.to_string());
                }
            }

            first += 1;
            second += 1;
        }

        return stack1 == stack2;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        first: String,
        second: String,
        expected: bool,
    }

    #[test]
    fn backspace_compare_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                first: "ab#c".to_string(),
                second: "ad#c".to_string(),
                expected: true,
            },
            TestCase {
                first: "ab##".to_string(),
                second: "c#d#".to_string(),
                expected: true,
            },
            TestCase {
                first: "a#c".to_string(),
                second: "b".to_string(),
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::backspace_compare(tc.first, tc.second));
        }
    }
}
