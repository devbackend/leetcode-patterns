struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![nums[0]];

        for i in 1..nums.len() {
            sums.push(sums[i - 1] + nums[i])
        }

        Self { sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut res = self.sums[right as usize];

        if left != 0 {
            res -= self.sums[left as usize - 1];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::sum_range::NumArray;

    struct TestCalc {
        left: i32,
        right: i32,
        expected: i32,
    }

    struct TestCase {
        nums: Vec<i32>,
        calcs: Vec<TestCalc>,
    }

    #[test]
    fn sum_range_test() {
        let test_cases = vec![
            TestCase{
                nums: vec![-2, 0, 3, -5, 2, -1],
                calcs: vec![
                    TestCalc{left: 0, right: 2, expected: 1},
                    TestCalc{left: 2, right: 5, expected: -1},
                    TestCalc{left: 0, right: 5, expected: -3},
                    TestCalc{left: 3, right: 5, expected: -4},
                ]
            }
        ];

        for tc in test_cases {
            let num_arr = NumArray::new(tc.nums);

            for c in tc.calcs {
                assert_eq!(c.expected, num_arr.sum_range(c.left, c.right));
            }


        }
    }
}
