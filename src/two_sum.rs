use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut res: Vec<i32> = Vec::new();
        let mut mem: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];

            match mem.get(&num) {
                Some(ix) => {
                    res.push(*ix);
                    res.push(i as i32);
                    break;
                }
                None => {
                    let diff = target - num;

                    mem.insert(diff, i as i32);
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
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    }

    #[test]
    fn two_sum_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![2, 7, 11, 15], target: 9, expected: vec![0, 1] },
            TestCase { nums: vec![3, 2, 4], target: 6, expected: vec![1, 2] },
            TestCase { nums: vec![3, 3], target: 6, expected: vec![0, 1] },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::two_sum(tc.nums, tc.target));
        }
    }
}
