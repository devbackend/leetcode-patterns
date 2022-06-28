use crate::Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l: i32 = 0;
        let mut r: i32 = (letters.len() - 1) as i32;

        while l <= r {
            let pos = (l + (r - l) / 2) as usize;

            if letters[pos] > target {
                r = pos as i32 - 1;
            } else {
                l = pos as i32 + 1;
            }
        }

        if l as usize == letters.len() {
            l = 0;
        }

        return letters[l as usize];
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        letters: Vec<char>,
        target: char,
        expected: char,
    }

    #[test]
    fn next_greatest_letter_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { letters: vec!['c', 'f', 'j'], target: 'a', expected: 'c' },
            TestCase { letters: vec!['c', 'f', 'j'], target: 'c', expected: 'f' },
            TestCase { letters: vec!['c', 'f', 'j'], target: 'd', expected: 'f' },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::next_greatest_letter(tc.letters, tc.target));
        }
    }
}
