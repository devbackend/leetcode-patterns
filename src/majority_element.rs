use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut mem: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            match mem.get(&num) {
                Some(count) => {
                    let count = count + 1;

                    mem.insert(num, count);
                }
                None => {
                    mem.insert(num, 1);
                }
            }
        }

        let mut mem: Vec<(i32, i32)> = mem.into_iter().collect();

        mem.sort_by(|a,b| b.1.cmp(&a.1));

        return mem[0].0;
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
    fn majority_element_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { nums: vec![3, 2, 3], expected: 3 },
            TestCase { nums: vec![2, 2, 1, 1, 1, 2, 2], expected: 2 },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::majority_element(tc.nums));
        }
    }
}
