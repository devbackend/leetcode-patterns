use crate::Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }

        let mut nums = nums;
        nums.sort();
        nums.reverse();

        let target = sum / k;

        let mut used = vec![false; nums.len()];

        Self::can_partition_k_subsets_rec(0, k, 0, &target, &mut used, &nums)
    }

    fn can_partition_k_subsets_rec(ix: usize, k: i32, chunk_sum: i32, target: &i32, mem: &mut Vec<bool>, used: &Vec<i32>) -> bool {
        if k == 0 {
            return true;
        }

        if chunk_sum == *target {
            return Self::can_partition_k_subsets_rec(0, k - 1, 0, target, mem, used);
        }

        for i in ix..used.len() {
            if mem[i] || used[i] + chunk_sum > *target {
                continue;
            }

            mem[i] = true;

            if Self::can_partition_k_subsets_rec(i + 1, k, chunk_sum + used[i], target, mem, used) {
                return true;
            }

            mem[i] = false;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        nums: Vec<i32>,
        k: i32,
        expected: bool,
    }

    #[test]
    fn can_partition_k_subsets_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![6, 4, 5, 4, 1, 1, 10, 1, 7, 6, 4, 2, 10, 1, 3, 5],
                k: 10,
                expected: false,
            },
            TestCase {
                nums: vec![4, 3, 2, 3, 5, 2, 1],
                k: 4,
                expected: true,
            },
            TestCase {
                nums: vec![1, 2, 3, 4],
                k: 3,
                expected: false,
            },
            TestCase {
                nums: vec![1, 2, 3],
                k: 2,
                expected: true,
            },
            TestCase {
                nums: vec![1, 2, 3, 4, 5, 6],
                k: 3,
                expected: true,
            },
            TestCase {
                nums: vec![5, 2, 2, 1, 2],
                k: 3,
                expected: false,
            },
        ];

        for tc in test_cases {
            println!("run {:?}", tc.nums);
            assert_eq!(tc.expected, Solution::can_partition_k_subsets(tc.nums, tc.k));
        }
    }
}
