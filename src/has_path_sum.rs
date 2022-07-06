use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root = root.unwrap();
        let val = root.borrow().val;
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        if left.is_none() && right.is_none() {
            return val == target_sum;
        }

        let target_sum = target_sum - val;

        return Solution::has_path_sum(left, target_sum) || Solution::has_path_sum(right, target_sum);
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
        target_sum: i32,
        expected: bool,
    }

    #[test]
    fn has_path_sum_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                root: None,
                target_sum: 21,
                expected: false,
            },
            TestCase {
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
                target_sum: 30,
                expected: true,
            },
            TestCase {
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
                target_sum: 102,
                expected: false,
            },
            TestCase {
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
                                left: None,
                                right: None,
                            }
                        ),
                    ),
                ),
                target_sum: 1,
                expected: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::has_path_sum(tc.root, tc.target_sum));
        }
    }
}
