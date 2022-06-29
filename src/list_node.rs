#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    #[inline]
    pub fn new_box(val: i32, next: Option<Box<Self>>) -> Box<Self> {
        Box::new(Self{val, next})
    }

    pub fn new_from_vec(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut head = Some(Self::new(0));

        let mut buf = head.as_mut().unwrap();
        for i in 0..nums.len() {
            buf.next = Some(Box::new(Self::new(nums[i])));

            buf = buf.next.as_mut().unwrap();
        }

        return head.unwrap().next
    }
}