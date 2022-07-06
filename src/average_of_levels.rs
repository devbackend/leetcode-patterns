use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
   use std::collections::vec_deque::VecDeque;

        let mut res: Vec<f64> = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while queue.len() > 0 {
            let len = queue.len();
            let mut sum = 0.0;

            for _ in 0..len {
                let item = queue.pop_front().unwrap().unwrap();
                let item = item.borrow();

                sum += item.val as f64;

                if item.left.is_some() {
                    queue.push_back(item.left.clone());
                }

                if item.right.is_some() {
                    queue.push_back(item.right.clone());
                }
            }

            res.push(sum / (len as f64));
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::average_of_levels::TreeNode;
    use crate::Solution;

    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        expected: Vec<f64>,
    }

    #[test]
    fn average_of_levels_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expected: vec![3.00000, 14.50000, 11.00000],
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 3,
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
                                                val: 20,
                                                left: Some(
                                                    Rc::new(
                                                        RefCell::new(
                                                            TreeNode {
                                                                val: 15,
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
                                                                val: 7,
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
            }
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::average_of_levels(tc.root));
        }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
