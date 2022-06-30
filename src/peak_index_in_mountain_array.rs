use crate::Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            return 0;
        }

        let mut first = 0;
        let mut second = 1;

        while second < arr.len() {
            if arr[first] > arr[second] {
                break;
            }

            first += 1;
            second += 1;
        }

        return first as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn peak_index_in_mountain_array_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2, 1, 0], expected: 0 },
            TestCase { nums: vec![0, 1, 0], expected: 1 },
            TestCase { nums: vec![0, 1, 2, 0], expected: 2 },
            TestCase { nums: vec![0, 5, 7, 10, 2], expected: 3 },
            TestCase { nums: vec![0, 5, 7, 9, 10], expected: 4 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::peak_index_in_mountain_array(tc.nums));
        }
    }
}
