use crate::Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;

        while i < nums.len() {
            let pos: usize = nums[i] as usize - 1;

            if nums[i] != nums[pos] {
                let buf = nums[i];
                nums[i] = nums[pos];
                nums[pos] = buf;
            } else {
                i += 1;
            }
        }

        let mut res: Vec<i32> = Vec::new();

        for n in 0..nums.len() {
            if nums[n] != n as i32 + 1 {
                res.push(n as i32 + 1);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<i32>,
    }

    #[test]
    fn find_disappeared_numbers_test() {
        let test_cases = vec![
            TestCase { nums: vec![4, 3, 2, 7, 8, 2, 3, 1], expected: vec![5, 6] },
            TestCase { nums: vec![1, 1], expected: vec![2] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::find_disappeared_numbers(tc.nums));
        }
    }
}