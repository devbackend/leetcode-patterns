use crate::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut left = 0;
        let mut right = matrix.len() - 1;

        while left < right {
            for i in 0..(right - left) {
                let top = left;
                let bottom = right;

                let top_left = matrix[top][left + i];

                matrix[top][left + i] = matrix[bottom - i][left];
                matrix[bottom - i][left] = matrix[bottom][right - i];
                matrix[bottom][right - i] = matrix[top + i][right];
                matrix[top + i][right] = top_left;
            }

            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        expected: Vec<Vec<i32>>,
        matrix: Vec<Vec<i32>>,
    }

    #[test]
    fn rotate_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], expected: vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]] },
            TestCase { matrix: vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]], expected: vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]] },
        ];

        for tc in test_cases {
            let mut matrix = tc.matrix.clone();

            Solution::rotate(&mut matrix);

            assert_eq!(tc.expected, matrix);
        }
    }
}
