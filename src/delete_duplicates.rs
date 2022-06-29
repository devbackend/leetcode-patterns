use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None
        }

        let mut head = head;

        let mut res = Some(ListNode::new(0));
        let mut buf = res.as_mut().unwrap();

        let mut prev = i32::MIN;

        while head.is_some() {
            if head.as_ref().unwrap().val != prev {
                buf.next = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
                buf = buf.next.as_mut().unwrap();

                prev = head.as_ref().unwrap().val;
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
        expected: Option<Box<ListNode>>,
    }

    #[test]
    fn delete_duplicates_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { head: ListNode::new_from_vec(vec![1, 1, 2, 3, 3, 4]), expected: ListNode::new_from_vec(vec![1, 2, 3, 4]) },
            TestCase { head: ListNode::new_from_vec(vec![1, 2, 3]), expected: ListNode::new_from_vec(vec![1, 2, 3]) },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::delete_duplicates(tc.head));
        }
    }
}
