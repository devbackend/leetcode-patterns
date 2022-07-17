use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::tree_node::TreeNode;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        Self::max_diameter(root, &mut max);

        return max;
    }

    fn max_diameter(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if root.is_none() {
            return -1;
        }

        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        let left_d = Self::max_diameter(left, max);
        let right_d = Self::max_diameter(right, max);

        *max = i32::max(*max, 2 + left_d + right_d);

        return 1 + i32::max(left_d, right_d);
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
    fn diameter_of_binary_tree_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expected: 4,
                root: TreeNode::new_from_vec(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]),
            },
            TestCase {
                expected: 1,
                root: TreeNode::new_from_vec(vec![Some(1), Some(2)]),
            },
            TestCase {
                expected: 2,
                root: TreeNode::new_from_vec(vec![Some(1), Some(2), Some(3)]),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, Solution::diameter_of_binary_tree(tc.root));
        }
    }
}
