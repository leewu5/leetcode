// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn from_vec(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut head: Option<Box<Self>> = None;
        let mut node = &mut head;
        for n in nums {
            *node = Some(Box::new(Self::new(n)));
            node = &mut node.as_mut().unwrap().next;
        }
        return head;
    }
}

pub fn display(list: &Option<Box<ListNode>>) {
    let mut node = list;
    while let Some(n) = node {
        print!("{}->", n.val);
        node = &n.next;
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut head = None;
    let mut l3 = &mut head;
    let mut c = 0;
    while l1 != None || l2 != None {
        let a = if let Some(node) = l1 {
            l1 = node.next;
            node.val
        } else {0};
        let b = if let Some(node) = l2 {
            l2 = node.next;
            node.val
        } else {0};
        *l3 = Some(Box::new(ListNode::new((a + b + c) % 10)));
        l3 = &mut l3.as_mut().unwrap().next;
        c = (a + b + c) / 10;
    }
    if c != 0 {*l3 = Some(Box::new(ListNode::new(c)));}
    return head;
}


