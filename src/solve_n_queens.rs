use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = Vec::new();

        let mut mem_col: HashMap<i32, bool> = HashMap::new();
        let mut mem_up_diag: HashMap<i32, bool> = HashMap::new();
        let mut mem_down_diag: HashMap<i32, bool> = HashMap::new();

        let mut board = Vec::new();

        Self::solve_n_queens_rec(n, 0, &mut res, &mut board, &mut mem_col, &mut mem_up_diag, &mut mem_down_diag);

        return res;
    }

    fn solve_n_queens_rec(
        n: i32,
        row: usize,
        res: &mut Vec<Vec<String>>,
        positions: &mut Vec<i32>,
        mem_col: &mut HashMap<i32, bool>,
        mem_up_diag: &mut HashMap<i32, bool>,
        mem_down_diag: &mut HashMap<i32, bool>,
    ) {
        if row == (n as usize) {
            let mut board: Vec<String> = Vec::new();

            for r in 0..positions.len() {
                let col = positions[r];

                let mut row = String::new();
                for c in 0..n {
                    if c == col {
                        row.push('Q');
                        continue;
                    }

                    row.push('.');
                }

                board.push(row);
            }

            res.push(board);
            return;
        }

        for col in 0..n {
            let up_diag = row as i32 - col;
            let down_diag = row as i32 + col;

            if mem_col.get(&col).is_some() || mem_up_diag.get(&up_diag).is_some() || mem_down_diag.get(&down_diag).is_some() {
                continue;
            }

            mem_col.insert(col, true);
            mem_up_diag.insert(up_diag, true);
            mem_down_diag.insert(down_diag, true);
            positions.push(col);

            Self::solve_n_queens_rec(n, row + 1, res, positions, mem_col, mem_up_diag, mem_down_diag);

            positions.pop();
            mem_col.remove(&col);
            mem_up_diag.remove(&up_diag);
            mem_down_diag.remove(&down_diag);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        n: i32,
        expected: Vec<Vec<String>>,
    }

    #[test]
    fn solve_n_queens_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                n: 1,
                expected: vec![
                    vec![String::from("Q")],
                ],
            },
            TestCase {
                n: 4,
                expected: vec![
                    vec![String::from(".Q.."), String::from("...Q"), String::from("Q..."), String::from("..Q.")],
                    vec![String::from("..Q."), String::from("Q..."), String::from("...Q"), String::from(".Q..")],
                ],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::solve_n_queens(tc.n));
        }
    }
}
