use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.as_ref().is_none() {
            return false
        }

        let mid = Solution::middle_node(head.clone());

        let mut reversed = Solution::reverse_list(mid);

        while reversed.as_ref().is_some() {
            if reversed.as_ref().unwrap().val != head.as_ref().unwrap().val {
                return false
            }

            reversed = reversed.unwrap().next.take();
            head = head.unwrap().next.take();
        }

        return true
    }
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome::ListNode;
    use crate::Solution;

    struct TestCase {
        head: Option<Box<ListNode>>,
        expected: bool,
    }

    #[test]
    fn is_palindrome_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expected: false,
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
            },
            TestCase {
                expected: true,
                head: Option::Some(
                    ListNode::new_box(
                        1,
                        Option::Some(
                            ListNode::new_box(
                                2,
                                Option::Some(
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
                            )
                        ),
                    )
                ),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::is_palindrome(tc.head));
        }
    }
}
