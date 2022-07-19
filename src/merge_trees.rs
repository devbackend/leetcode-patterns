use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() && root2.is_none() {
            return None
        }

        let mut tree_node = TreeNode::new(0);

        let mut left1: Option<Rc<RefCell<TreeNode>>> = None;
        let mut left2: Option<Rc<RefCell<TreeNode>>> = None;
        let mut right1: Option<Rc<RefCell<TreeNode>>> = None;
        let mut right2: Option<Rc<RefCell<TreeNode>>> = None;

        if root1.is_some() {
            let root = root1.unwrap();

            left1 = root.borrow().left.clone();
            right1 = root.borrow().right.clone();

            tree_node.val += root.borrow().val;
        }

        if root2.is_some() {
            let root = root2.unwrap();

            left2 = root.borrow().left.clone();
            right2 = root.borrow().right.clone();

            tree_node.val += root.borrow().val;
        }

        tree_node.left = Self::merge_trees(left1, left2);
        tree_node.right = Self::merge_trees(right1, right2);

        return Some(Rc::new(RefCell::new(tree_node)));
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Solution;
    use crate::tree_node::TreeNode;

    struct TestCase {
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
        expected: Option<Rc<RefCell<TreeNode>>>,
    }

    #[test]
    fn merge_trees_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                root1: TreeNode::new_from_vec(vec![Some(1), Some(3), Some(2), Some(5)]),
                root2: TreeNode::new_from_vec(vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)]),
                expected: TreeNode::new_from_vec(vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)]),
            },
            TestCase {
                root1: TreeNode::new_from_vec(vec![Some(1)]),
                root2: TreeNode::new_from_vec(vec![Some(1), Some(2)]),
                expected: TreeNode::new_from_vec(vec![Some(2), Some(2)]),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::merge_trees(tc.root1, tc.root2));
        }
    }
}
