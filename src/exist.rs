use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let mut mem: HashMap<(usize, usize), bool> = HashMap::new();

        for r in 0..board.len() {
            for c in 0..board[r].len() {
                let mut board = board.clone();

                if Self::exists_rec(&mut board, &chars, 0, (r, c), &mut mem) {
                    return true;
                }
            }
        }

        return false;
    }

    fn exists_rec(board: &mut Vec<Vec<char>>, word: &Vec<char>, ix: usize, position: (usize, usize), mem: &mut HashMap<(usize, usize), bool>) -> bool {
        if ix >= word.len() {
            return true;
        }

        if position.0 >= board.len() || position.1 >= board[position.0].len() {
            return false;
        }

        if board[position.0][position.1] != word[ix] {
            return false;
        }

        if mem.get(&position).is_some() {
            return false
        }

        mem.insert(position, true);

        let ix = ix + 1;

        let mut res = false;

        if position.0 != 0 && Self::exists_rec(board, word, ix, (position.0 - 1, position.1), mem) {
            res = true;
        }

        if position.1 != 0 && Self::exists_rec(board, word, ix, (position.0, position.1 - 1), mem) {
            res = true;
        }

        if Self::exists_rec(board, word, ix, (position.0 + 1, position.1), mem) {
            res = true;
        }

        if Self::exists_rec(board, word, ix, (position.0, position.1 + 1), mem) {
            res = true;
        }

        mem.remove(&position);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        board: Vec<Vec<char>>,
        word: String,
        expected: bool,
    }

    #[test]
    fn exist_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'E', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "ABCESEEEFS".to_string(),
                expected: true,
            },
            TestCase {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "ABCCED".to_string(),
                expected: true,
            },
            TestCase {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "SEE".to_string(),
                expected: true,
            },
            TestCase {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "ABA".to_string(),
                expected: false,
            },
            TestCase {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "ABCB".to_string(),
                expected: false,
            },
            TestCase {
                board: vec![
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                    vec!['A', 'A', 'A', 'A', 'A', 'A'],
                ],
                word: "AAAAAAAAAAAABAA".to_string(),
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::exist(tc.board, tc.word));
        }
    }
}
