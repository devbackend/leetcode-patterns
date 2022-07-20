use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || p.is_none() || q.is_none() {
            return None
        }

        let left = p.as_ref().unwrap().borrow().val;
        let right = q.as_ref().unwrap().borrow().val;

        if left > right {
            return Self::lowest_common_ancestor(root, q, p);
        }

        let root = root.unwrap();
        let root_val = root.borrow().val;

        if left <= root_val && root_val <= right {
            return Some(root);
        }

        if left > root_val {
            return Self::lowest_common_ancestor(root.borrow().right.clone(), p, q);
        }

        return Self::lowest_common_ancestor(root.borrow().left.clone(), p, q);
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
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
        expected: i32,
    }

    #[test]
    fn lowest_common_ancestor_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]),
                p: TreeNode::new_from_vec(vec![Some(2)]),
                q: TreeNode::new_from_vec(vec![Some(8)]),
                expected: 6,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]),
                p: TreeNode::new_from_vec(vec![Some(4)]),
                q: TreeNode::new_from_vec(vec![Some(2)]),
                expected: 2,
            },
            TestCase {
                root: TreeNode::new_from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]),
                p: TreeNode::new_from_vec(vec![Some(3)]),
                q: TreeNode::new_from_vec(vec![Some(9)]),
                expected: 6,
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::lowest_common_ancestor(tc.root, tc.p, tc.q).unwrap().borrow().val);
        }
    }
}
