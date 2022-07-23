use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::subsets_rec(&nums, 0)
    }

    fn subsets_rec(nums: &Vec<i32>, ix: usize) -> Vec<Vec<i32>> {
        if ix == nums.len() - 1 {
            return vec![vec![], vec![nums[ix]]]
        }

        let mut subsets = Self::subsets_rec(nums, ix + 1);

        for i in 0..subsets.len() {
            let mut candidate = vec![nums[ix]];

            candidate.append(subsets[i].clone().as_mut());

            subsets.push(candidate)
        }

        return subsets
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
    fn subsets_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![1, 2, 3],
                expected: vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]],
            },
            TestCase {
                nums: vec![1, 2],
                expected: vec![vec![], vec![1], vec![2], vec![1, 2]],
            },
            TestCase {
                nums: vec![10],
                expected: vec![vec![], vec![10]],
            },
        ];

        for tc in test_cases {
            let mut actual = Solution::subsets(tc.nums);
            let mut expected = tc.expected;

            actual.sort();
            expected.sort();

            assert_eq!(expected, actual);
        }
    }
}
