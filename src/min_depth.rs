use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }

        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        if right.is_none() && left.is_some() {
            return 1 + Solution::min_depth(left);
        }

        if left.is_none() && right.is_some() {
            return 1 + Solution::min_depth(right);
        }

        let left_depth = 1 + Solution::min_depth(left);
        let right_depth = 1 + Solution::min_depth(right);

        return if left_depth > right_depth {
            right_depth
        } else {
            left_depth
        }
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
        expected: i32,
    }

    #[test]
    fn min_depth_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                expected: 2,
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
                                left: Some(
                                    Rc::new(
                                        RefCell::new(
                                            TreeNode {
                                                val: 2,
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
                                                val: 3,
                                                left: Some(
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
                                                right: Some(
                                                    Rc::new(
                                                        RefCell::new(
                                                            TreeNode {
                                                                val: 5,
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
            TestCase{
                expected: 5,
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
                                right: None,
                                left: Some(
                                    Rc::new(
                                        RefCell::new(
                                            TreeNode {
                                                val: 2,
                                                left: None,
                                                right: Some(
                                                    Rc::new(
                                                        RefCell::new(
                                                            TreeNode {
                                                                val: 3,
                                                                left: None,
                                                                right: Some(
                                                                    Rc::new(
                                                                        RefCell::new(
                                                                            TreeNode {
                                                                                val: 4,
                                                                                left: None,
                                                                                right: Some(
                                                                                    Rc::new(
                                                                                        RefCell::new(
                                                                                            TreeNode {
                                                                                                val: 5,
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
                                        ),
                                    ),
                                ),
                            }
                        ),
                    ),
                ),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::min_depth(tc.root));
        }
    }
}
