use std::arch::x86_64::_mm256_floor_pd;

use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }

        let mut lists = lists;

        let mut res = Some(ListNode::new(0));
        let mut buf = res.as_mut().unwrap();

        loop {
            let mut pos: i32 = -1;
            let mut min: i32 = i32::MAX;

            for i in 0..lists.len() {
                if lists[i].is_none() {
                    continue;
                }

                let list = lists[i].as_ref().unwrap();

                if list.val < min {
                    min = list.val;
                    pos = i as i32;
                }
            }

            if pos == -1 {
                break;
            }

            buf.next = Some(Box::new(ListNode::new(min)));
            buf = buf.next.as_mut().unwrap();
            lists[pos as usize] = lists[pos as usize].clone().unwrap().next;
        }

        return res.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;
    use crate::Solution;

    struct TestCase {
        lists: Vec<Option<Box<ListNode>>>,
        expected: Option<Box<ListNode>>,
    }

    #[test]
    fn merge_k_lists_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expected: ListNode::new_from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]),
                lists: vec![
                    ListNode::new_from_vec(vec![1, 4, 5]),
                    ListNode::new_from_vec(vec![1, 3, 4]),
                    ListNode::new_from_vec(vec![2, 6]),
                ],
            },
            TestCase {
                expected: ListNode::new_from_vec(vec![]),
                lists: vec![],
            },
            TestCase {
                expected: ListNode::new_from_vec(vec![]),
                lists: vec![ListNode::new_from_vec(vec![])],
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::merge_k_lists(tc.lists));
        }
    }
}
