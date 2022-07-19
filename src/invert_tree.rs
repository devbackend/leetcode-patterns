use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root
        }

        let root = root.unwrap();
        let left = Self::invert_tree(root.borrow().right.clone());
        let right = Self::invert_tree(root.borrow().left.clone());

        return Some(Rc::new(RefCell::new(TreeNode{val: root.borrow().val, left, right})));
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
        expected: Option<Rc<RefCell<TreeNode>>>,
    }

    #[test]
    fn invert_tree_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase { root: TreeNode::new_from_vec(vec![Some(1), Some(2), Some(3)]), expected: TreeNode::new_from_vec(vec![Some(1), Some(3), Some(2)]) },
            TestCase { root: TreeNode::new_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]), expected: TreeNode::new_from_vec(vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]) },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::invert_tree(tc.root));
        }
    }
}
