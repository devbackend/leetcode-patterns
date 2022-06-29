use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head == None {
            return None
        }

        let mut head = head;

        let mut res = Some(ListNode::new(0));
        let mut buf = res.as_mut().unwrap();

        while head.is_some() {
            if head.as_ref().unwrap().val != val {
                buf.next = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
                buf = buf.next.as_mut().unwrap();
            }

            head = head.unwrap().next;
        }

        return res.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::Solution;

    struct TestCase {
        head: Option<Box<ListNode>>,
        val: i32,
        expected: Option<Box<ListNode>>,
    }

    #[test]
    fn remove_elements_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                head: ListNode::new_from_vec(vec![1, 2, 6, 3, 4, 5, 6]),
                val: 6,
                expected: ListNode::new_from_vec(vec![1, 2, 3, 4, 5]),
            },
            TestCase {
                head: ListNode::new_from_vec(vec![1, 2, 3, 4, 5]),
                val: -1,
                expected: ListNode::new_from_vec(vec![1, 2, 3, 4, 5]),
            },
            TestCase {
                head: ListNode::new_from_vec(Vec::new()),
                val: 1,
                expected: ListNode::new_from_vec(Vec::new()),
            },
            TestCase {
                head: ListNode::new_from_vec(vec![1, 1, 1, 1, 1]),
                val: 1,
                expected: ListNode::new_from_vec(Vec::new()),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::remove_elements(tc.head, tc.val));
        }
    }
}
