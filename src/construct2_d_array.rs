use crate::Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m < 1 || n < 1 {
            return Vec::new();
        }

        let m: usize = m as usize;
        let n: usize = n as usize;

        if m * n != original.len() {
            return Vec::new();
        }

        let mut res = Vec::new();

        for i in 0..original.len() {
            let row = i / n;

            if res.len() == row {
                res.push(Vec::new());
            }

            res[row].push(original[i]);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        m: i32,
        n: i32,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn construct2_d_array_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { expected: vec![vec![1, 2], vec![3, 4]], nums: vec![1, 2, 3, 4], m: 2, n: 2 },
            TestCase { expected: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]], nums: vec![1, 2, 3, 4, 5, 6, 7, 8], m: 2, n: 4 },
            TestCase { expected: vec![], nums: vec![1, 2, 3, 4], m: 1, n: 2 },
            TestCase { expected: vec![], nums: vec![1, 2, 3, 4], m: -2, n: 2 },
            TestCase { expected: vec![], nums: vec![1, 2, 3, 4], m: 2, n: -2 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::construct2_d_array(tc.nums, tc.m, tc.n));
        }
    }
}
