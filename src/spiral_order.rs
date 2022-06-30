use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Direction {
            Left,
            Bottom,
            Right,
            Top,
        }

        let mut res = vec![];

        let mut top = 0;
        let mut bottom = matrix.len();
        let mut left = 0;
        let mut right = matrix[0].len();

        let mut direction = Direction::Left;

        while top < bottom && left < right {
            match &direction {
                Direction::Left => {
                    for i in left..right {
                        res.push(matrix[left][i]);
                    }

                    top += 1;
                    direction = Direction::Bottom;
                }
                Direction::Bottom => {
                    for i in top..bottom {
                        res.push(matrix[i][right - 1]);
                    }

                    right -= 1;
                    direction = Direction::Right;
                }
                Direction::Right => {
                    for i in (left..right).rev() {
                        res.push(matrix[bottom - 1][i]);
                    }

                    bottom -= 1;
                    direction = Direction::Top;
                }
                Direction::Top => {
                    for i in (top..bottom).rev() {
                        res.push(matrix[i][left]);
                    }

                    left += 1;
                    direction = Direction::Left;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        matrix: Vec<Vec<i32>>,
        expected: Vec<i32>,
    }

    #[test]
    fn spiral_order_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], expected: vec![1, 2, 3, 6, 9, 8, 7, 4, 5] },
            TestCase { matrix: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]], expected: vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::spiral_order(tc.matrix));
        }
    }
}
