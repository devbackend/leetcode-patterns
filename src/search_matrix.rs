use crate::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        if target < matrix[0][0] || target > matrix[m - 1][n - 1] {
            return false;
        }

        let mut l = 0;
        let mut r = m * n;

        while l <= r {
            let pos = (l + (r - l) / 2) as usize;

            let row = pos / n;
            let cell = pos % n;

            if matrix[row][cell] == target {
                return true;
            }

            if matrix[row][cell] > target {
                r = pos - 1;
            } else {
                l = pos + 1;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        matrix: Vec<Vec<i32>>,
        target: i32,
        expected: bool,
    }

    #[test]
    fn search_matrix_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 3,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 13,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 16,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 25,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 0,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 79,
                expected: false,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 10,
                expected: true,
            },
            TestCase {
                matrix: vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                target: 60,
                expected: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::search_matrix(tc.matrix, tc.target));
        }
    }
}
