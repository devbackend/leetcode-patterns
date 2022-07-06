use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn is_same_tree(first: Option<Rc<RefCell<TreeNode>>>, second: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::vec_deque::VecDeque;

        let mut queue = VecDeque::new();

        queue.push_back(first);
        queue.push_back(second);

        while queue.len() > 0 {
            let f = queue.pop_front().unwrap();
            let s = queue.pop_front().unwrap();

            if f.is_none() && s.is_none() {
                continue;
            }

            if (f.is_some() && s.is_none()) || (f.is_none() && s.is_some()) {
                return false;
            }

            let f_item = f.unwrap();
            let s_item = s.unwrap();

            let f_item = f_item.borrow();
            let s_item = s_item.borrow();

            if f_item.val != s_item.val {
                return false;
            }

            queue.push_back(f_item.left.clone());
            queue.push_back(s_item.left.clone());

            queue.push_back(f_item.right.clone());
            queue.push_back(s_item.right.clone());
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Solution;
    use crate::tree_node::TreeNode;

    struct TestCase {
        first: Option<Rc<RefCell<TreeNode>>>,
        second: Option<Rc<RefCell<TreeNode>>>,
        expected: bool,
    }

    #[test]
    fn is_same_tree_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                expected: true,
                first: Some(
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
                second: Some(
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
            TestCase{
                expected: false,
                first: Some(
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
                second: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
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
                                                val: 2,
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
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::is_same_tree(tc.first, tc.second));
        }
    }
}
