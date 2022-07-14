use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        use std::collections::vec_deque::VecDeque;

        let mut res = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while queue.len() > 0 {
            let len = queue.len();
            let mut nums = Vec::new();

            for _ in 0..len {
                let item = queue.pop_front().unwrap().unwrap();
                let item = item.borrow();

                nums.push(item.val);

                if item.left.is_some() {
                    queue.push_back(item.left.clone());
                }

                if item.right.is_some() {
                    queue.push_back(item.right.clone());
                }
            }

            res.push(nums);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Solution;
    use crate::tree_node::TreeNode;

    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn level_order_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expected: vec![vec![1], vec![9, 2], vec![3, 4]],
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
                                left: Some(
                                    Rc::new(
                                        RefCell::new(
                                            TreeNode {
                                                val: 9,
                                                left: None,
                                                right: None,
                                            }
                                        ),
                                    ),
                                ),
                                right: Some(
                                    Rc::new(
                                        RefCell::new(
                                            TreeNode {
                                                val: 2,
                                                left: Some(
                                                    Rc::new(
                                                        RefCell::new(
                                                            TreeNode {
                                                                val: 3,
                                                                left: None,
                                                                right: None,
                                                            }
                                                        ),
                                                    ),
                                                ),
                                                right: Some(
                                                    Rc::new(
                                                        RefCell::new(
                                                            TreeNode {
                                                                val: 4,
                                                                left: None,
                                                                right: None,
                                                            }
                                                        ),
                                                    ),
                                                ),
                                            }
                                        ),
                                    ),
                                ),
                            }
                        ),
                    ),
                ),
            },
            TestCase {
                expected: vec![],
                root: None,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::level_order(tc.root));
        }
    }
}
