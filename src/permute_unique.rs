use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut items = Vec::new();
        let mut mem: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];

            if mem.get(&num).is_none() {
                mem.insert(num, 1);
                continue;
            }

            mem.insert(num, *mem.get(&num).unwrap() + 1);
        }

        Self::permute_unique_rec(&mut res, &nums, &mut items, &mut mem);

        return res;
    }

    fn permute_unique_rec(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, items: &mut Vec<i32>, mem: &mut HashMap<i32, i32>) {
        if items.len() == nums.len() {
            res.push(items.clone());
            return;
        }

        for num in mem.clone().keys() {
            let count = *mem.get(&num).unwrap();

            if count == 0 {
                continue
            }

            items.push(*num);
            *mem.get_mut(num).unwrap() -= 1;

            Self::permute_unique_rec(res, nums, items, mem);

            *mem.get_mut(num).unwrap() += 1;
            items.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn permute_unique_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![1, 1, 2],
                expected: vec![
                    vec![1, 1, 2],
                    vec![1, 2, 1],
                    vec![2, 1, 1],
                ],
            },
            TestCase {
                nums: vec![1, 2, 3],
                expected: vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::permute_unique(tc.nums);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
