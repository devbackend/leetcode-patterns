use crate::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows: Vec<usize> = Vec::new();
        let mut columns: Vec<usize> = Vec::new();

        for r in 0..matrix.len() {
            for c in 0..matrix[r].len() {
                if matrix[r][c] == 0 {
                    rows.push(r);
                    columns.push(c);
                }
            }
        }

        for i in 0..rows.len() {
            let r = rows[i];

            matrix[r] = vec![0; matrix[r].len()];
        }

        for j in 0..columns.len() {
            let c = columns[j];

            for i in 0..matrix.len() {
                matrix[i][c] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        matrix: Vec<Vec<i32>>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn set_zeroes_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { matrix: vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], expected: vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]] },
            TestCase { matrix: vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]], expected: vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]] },
        ];

        for tc in test_cases {
            let mut matrix = tc.matrix;

            Solution::set_zeroes(&mut matrix);

            assert_eq!(tc.expected, matrix);
        }
    }
}
