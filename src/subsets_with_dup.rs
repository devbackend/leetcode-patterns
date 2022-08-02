use crate::Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut items = Vec::new();

        let mut nums = nums;
        nums.sort();

        Self::subsets_with_dup_rec(&nums, 0, &mut items, &mut res);

        return res
    }

    fn subsets_with_dup_rec(nums: &Vec<i32>, ix: usize, items: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if ix == nums.len() {
            res.push(items.clone());
            return;
        }

        items.push(nums[ix]);
        Self::subsets_with_dup_rec(nums, ix + 1, items, res);
        items.pop();

        let mut ix = ix;

        while ix + 1 < nums.len() && nums[ix] == nums[ix + 1] {
            ix += 1;
        }

        Self::subsets_with_dup_rec(nums, ix + 1, items, res);
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
    fn subsets_with_dup_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![1, 2, 2],
                expected: vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]],
            },
            TestCase {
                nums: vec![0],
                expected: vec![vec![], vec![0]],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::subsets_with_dup(tc.nums);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
