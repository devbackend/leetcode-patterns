use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut slow = head.as_ref().unwrap().as_ref();
        let mut fast = head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();

        while fast.as_ref().is_some() {
            if fast.as_ref().unwrap().next.is_none() {
                break;
            }

            slow = slow.next.as_ref().unwrap();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        return Option::Some(slow.next.as_ref().unwrap().clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::Solution;

    #[test]
    fn middle_node_two_nodes_test() {
        let node = Option::Some(
            ListNode::new_box(
                1,
                Option::Some(
                    ListNode::new_box(
                        2,
                        Option::None,
                    )
                ),
            )
        );

        assert_eq!(node.as_ref().unwrap().next, Solution::middle_node(node.clone()))
    }

    #[test]
    fn middle_node_odd_nodes_test() {
        let node = Option::Some(
            ListNode::new_box(
                1,
                Option::Some(
                    ListNode::new_box(
                        2,
                        Option::Some(
                            ListNode::new_box(
                                3,
                                Option::Some(
                                    ListNode::new_box(
                                        4,
                                        Option::Some(
                                            ListNode::new_box(
                                                5,
                                                Option::None,
                                            )
                                        ),
                                    )
                                ),
                            )
                        ),
                    )
                ),
            )
        );

        assert_eq!(node.as_ref().unwrap().next.as_ref().unwrap().next, Solution::middle_node(node.clone()))
    }

    #[test]
    fn middle_node_even_nodes_test() {
        let node = Option::Some(
            ListNode::new_box(
                1,
                Option::Some(
                    ListNode::new_box(
                        2,
                        Option::Some(
                            ListNode::new_box(
                                3,
                                Option::Some(
                                    ListNode::new_box(
                                        4,
                                        Option::None,
                                    )
                                ),
                            )
                        ),
                    )
                ),
            )
        );

        assert_eq!(node.as_ref().unwrap().next.as_ref().unwrap().next, Solution::middle_node(node.clone()))
    }
}
