// Definition for singly-linked list.
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

    /// 将数组转换为链表，数组中的元素按从头到尾的顺序存储。
    pub fn from_vec(values: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        let mut tail = &mut head;
        for value in values {
            *tail = Some(Box::new(Self::new(value)));
            tail = &mut tail.as_mut().expect("刚刚插入的节点必须存在").next;
        }
        head
    }

    /// 将链表转换回数组，便于编写测试和查看结果。
    pub fn to_vec(list: &Option<Box<Self>>) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = list.as_deref();
        while let Some(node) = current {
            values.push(node.val);
            current = node.next.as_deref();
        }
        values
    }
}

/// 合并 k 个升序链表。
///
/// TODO: 完成题解实现。建议思考顺序：顺序合并、分治合并或最小堆。
pub fn merge_k_lists(_lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    todo!("请实现第 23 题：Merge k Sorted Lists")
}
