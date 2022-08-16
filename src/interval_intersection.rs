use crate::Solution;

impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        for i in 0..first_list.len() {
            let first = first_list[i].clone();
            for j in 0..second_list.len() {
                let second = second_list[j].clone();

                if first[1] < second[0] {
                    break;
                }

                let start = i32::max(first[0], second[0]);
                let end = i32::min(first[1], second[1]);

                if start <= end {
                    res.push(vec![start, end]);
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    struct TestCase {
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn interval_intersection_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                first_list:  vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                second_list: vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
                expected: vec![vec![1, 2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]],
            },
            TestCase {
                first_list: vec![vec![1, 3], vec![5, 9]],
                second_list: vec![],
                expected: vec![],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::interval_intersection(tc.first_list, tc.second_list));
        }
    }
}
