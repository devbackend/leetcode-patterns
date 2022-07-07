use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn new_from_vec(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::insert_level_order(nums, 0);
    }

    fn insert_level_order(nums: Vec<Option<i32>>, lvl: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;

        if lvl < nums.len() && nums[lvl].is_some() {
            let val = nums[lvl].unwrap();
            let left = Self::insert_level_order(nums.clone(), 2 * lvl + 1);
            let right = Self::insert_level_order(nums.clone(), 2 * lvl + 2);

            root = Some(Rc::new(RefCell::new(TreeNode{val, left, right})))
        }

        return root;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::tree_node::TreeNode;

    struct TestCase {
        nums: Vec<Option<i32>>,
        expected: Option<Rc<RefCell<TreeNode>>>,
    }

    #[test]
    fn new_from_vec_test() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![],
                expected: None,
            },
            TestCase {
                nums: vec![Some(1)],
                expected: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            },
            TestCase {
                nums: vec![Some(1), None, Some(2)],
                expected: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: 1,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                            }
                        )
                    )
                ),
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.expected, TreeNode::new_from_vec(tc.nums));
        }
    }
}