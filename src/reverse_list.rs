use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut head, mut prev) = (head, None);
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
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
    fn reverse_list_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                head: Option::Some(
                    ListNode::new_box(
                        1,
                        Option::Some(
                            ListNode::new_box(
                                2,
                                Option::None,
                            )
                        ),
                    )
                ),
                expected: Option::Some(
                    ListNode::new_box(
                        2,
                        Option::Some(
                            ListNode::new_box(
                                1,
                                Option::None,
                            )
                        ),
                    )
                ),
            }
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::reverse_list(tc.head));
        }
    }
}
