use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let nums: Vec<char> = s.chars().collect();
        let mut mem: HashMap<usize, i32> = HashMap::new();

        mem.insert(nums.len(), 1);

        Self::num_decodings_rec(&nums, 0, &mut mem)
    }

    fn num_decodings_rec(nums: &Vec<char>, ix: usize, mem: &mut HashMap<usize, i32>) -> i32 {
        if mem.get(&ix).is_some() {
            return *mem.get(&ix).unwrap();
        }

        if nums[ix] == '0' {
            return 0;
        }

        let mut res = Self::num_decodings_rec(nums, ix + 1, mem);

        if ix == nums.len() - 1 {
            return res;
        }

        let f: i32 = nums[ix].to_string().parse().unwrap();
        let s: i32 = nums[ix + 1].to_string().parse().unwrap();

        let num = f * 10 + s;
        if num < 27 {
            res += Self::num_decodings_rec(nums, ix + 2, mem)
        }

        mem.insert(ix, res);

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        s: String,
        expected: i32,
    }

    #[test]
    fn num_decodings_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { s: String::from("227"), expected: 2 },
            TestCase { s: String::from("06"), expected: 0 },
            TestCase { s: String::from("1234567890123456789"), expected: 0 },
            TestCase { s: String::from("12"), expected: 2 },
            TestCase { s: String::from("226"), expected: 3 },
            TestCase { s: String::from("10101"), expected: 1 },
            TestCase { s: String::from("1020"), expected: 1 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::num_decodings(tc.s));
        }
    }
}
