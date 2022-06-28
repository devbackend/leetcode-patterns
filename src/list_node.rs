#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    #[inline]
    pub fn new_box(val: i32, next: Option<Box<ListNode>>) -> Box<Self> {
        Box::new(ListNode{val, next})
    }
}