use crate::Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = heights.len() - 1;

        let mut max = 0;
        while l < r {
            max = i32::max(max, i32::min(heights[l], heights[r]) * (r - l) as i32);

            if heights[l] < heights[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        return max;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        heights: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn max_area_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                heights: vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
                expected: 49,
            },
            TestCase {
                heights: vec![1, 1],
                expected: 1,
            },
            TestCase {
                heights: vec![1, 2, 3, 50, 5, 3, 50, 4, 2, 1],
                expected: 150,
            },
            TestCase {
                heights: vec![3, 5, 1, 1, 1, 4, 3],
                expected: 18,
            },
            TestCase {
                heights: vec![2; 10000],
                expected: 19998,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::max_area(tc.heights));
        }
    }
}
