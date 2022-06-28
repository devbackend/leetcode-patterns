use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut list1 = list1;
        let mut list2 = list2;

        let mut dummy = Some(ListNode::new(-1));
        let mut buf = dummy.as_mut().unwrap();

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                buf.next = list1.clone();
                list1 = list1.unwrap().next;
            } else {
                buf.next = list2.clone();
                list2 = list2.unwrap().next;
            }

            buf = buf.next.as_mut().unwrap();
        }

        if list1.is_some() {
            buf.next = list1
        }

        if list2.is_some() {
            buf.next = list2
        }

        return dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::Solution;

    struct TestCase {
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        expected: Option<Box<ListNode>>,
    }

    #[test]
    fn merge_two_lists_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                list1: Some(
                    ListNode::new_box(
                        1,
                        Some(
                            ListNode::new_box(
                                2,
                                None,
                            )
                        ),
                    )
                ),
                list2: None,
                expected: Some(
                    ListNode::new_box(
                        1,
                        Some(
                            ListNode::new_box(
                                2,
                                None,
                            )
                        ),
                    )
                ),
            },
            TestCase {
                list1: None,
                list2: Some(
                    ListNode::new_box(
                        3,
                        Some(
                            ListNode::new_box(
                                4,
                                None,
                            )
                        ),
                    )
                ),
                expected: Some(
                    ListNode::new_box(
                        3,
                        Some(
                            ListNode::new_box(
                                4,
                                None,
                            )
                        ),
                    )
                ),
            },
            TestCase {
                list1: Some(
                    ListNode::new_box(
                        1,
                        Some(
                            ListNode::new_box(
                                2,
                                None,
                            )
                        ),
                    )
                ),
                list2: Some(
                    ListNode::new_box(
                        3,
                        Some(
                            ListNode::new_box(
                                4,
                                None,
                            )
                        ),
                    )
                ),
                expected: Some(
                    ListNode::new_box(
                        1,
                        Some(
                            ListNode::new_box(
                                2,
                                Some(
                                    ListNode::new_box(
                                        3,
                                        Some(
                                            ListNode::new_box(
                                                4,
                                                None,
                                            )
                                        ),
                                    )
                                ),
                            )
                        ),
                    )
                ),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::merge_two_lists(tc.list1, tc.list2));
        }
    }
}
