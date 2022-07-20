use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() || sub_root.is_none() {
            return root.is_none() && sub_root.is_none();
        }

        if Self::same(root.clone(), sub_root.clone()) {
            return true;
        }

        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        return Self::is_subtree(left, sub_root.clone()) || Self::is_subtree(right, sub_root.clone());
    }

    fn same(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() || sub_root.is_none() {
            return root.is_none() && sub_root.is_none();
        }

        let root = root.unwrap();
        let root_left = root.borrow().left.clone();
        let root_right = root.borrow().right.clone();

        let sub_root = sub_root.unwrap();
        let sub_root_left = sub_root.borrow().left.clone();
        let sub_root_right = sub_root.borrow().right.clone();

        if root.borrow().val != sub_root.borrow().val {
            return false;
        }

        return Self::same(root_left, sub_root_left) && Self::same(root_right, sub_root_right);
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
        sub_root: Option<Rc<RefCell<TreeNode>>>,
        expected: bool,
    }

    #[test]
    fn is_subtree_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), Some(2)]),
                sub_root: TreeNode::new_from_vec(vec![Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), None, Some(1), Some(2)]),
                expected: true,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(3), Some(4), Some(5), Some(1), None, Some(2)]),
                sub_root: TreeNode::new_from_vec(vec![Some(3), Some(1), Some(2)]),
                expected: false,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]),
                sub_root: TreeNode::new_from_vec(vec![Some(4), Some(1), Some(2)]),
                expected: true,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, Some(0)]),
                sub_root: TreeNode::new_from_vec(vec![Some(4), Some(1), Some(2)]),
                expected: false,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(4), Some(4), Some(5), Some(1), Some(2)]),
                sub_root: TreeNode::new_from_vec(vec![Some(4), Some(1), Some(2)]),
                expected: true,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::is_subtree(tc.root, tc.sub_root));
        }
    }
}
