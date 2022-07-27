use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let mut res: Vec<i32> = vec![(l1.as_ref().unwrap().val + l2.as_ref().unwrap().val) % 10];

        let mut sum = (l1.as_ref().unwrap().val + l2.as_ref().unwrap().val) / 10;

        let mut first = l1.unwrap().next;
        let mut second = l2.unwrap().next;

        while first.is_some() || second.is_some() {
            if first.is_some() {
                sum += first.as_ref().unwrap().val;
                first = first.unwrap().next;
            }

            if second.is_some() {
                sum += second.as_ref().unwrap().val;
                second = second.unwrap().next;
            }

            res.push(sum % 10);

            sum /= 10;
        }

        if sum > 0 {
            res.push(sum % 10);
        }

        return ListNode::new_from_vec(res);
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::Solution;

    struct TestCase {
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        expected: Option<Box<ListNode>>,
    }

    #[test]
    fn add_two_numbers_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                l1: ListNode::new_from_vec(vec![2, 4, 3]),
                l2: ListNode::new_from_vec(vec![5, 6, 4]),
                expected: ListNode::new_from_vec(vec![7, 0, 8]),
            },
            TestCase {
                l1: ListNode::new_from_vec(vec![0]),
                l2: ListNode::new_from_vec(vec![0]),
                expected: ListNode::new_from_vec(vec![0]),
            },
            TestCase {
                l1: ListNode::new_from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                l2: ListNode::new_from_vec(vec![9, 9, 9, 9]),
                expected: ListNode::new_from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::add_two_numbers(tc.l1, tc.l2));
        }
    }
}
