use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

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

/// 堆条目只比较节点值和来源链表编号，不需要为整条链表定义排序。
struct HeapEntry {
    list_index: usize,
    node: Box<ListNode>,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.node.val == other.node.val && self.list_index == other.list_index
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node
            .val
            .cmp(&other.node.val)
            .then_with(|| self.list_index.cmp(&other.list_index))
    }
}

/// 合并 k 个升序链表。
///
/// 最小堆始终保存每条非空链表当前尚未处理的最小节点。每次弹出全局最小节点，
/// 再把该节点所在链表的下一个节点放入堆，直到所有节点处理完毕。
///
/// 时间复杂度：`O(n log k)`，n 为节点总数，k 为链表数量。
/// 额外空间：`O(n + k)`，包括临时结果数组；堆最多保存 k 个节点。
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut min_heap = BinaryHeap::new();

    // 初始化堆：每条链表只放入头节点，空链表跳过。
    for (list_index, node) in lists.into_iter().enumerate() {
        if let Some(node) = node {
            min_heap.push(Reverse(HeapEntry { list_index, node }));
        }
    }

    let mut values = Vec::new();
    while let Some(Reverse(mut entry)) = min_heap.pop() {
        // 保留来源编号，并将该链表的下一个节点作为新的候选项入堆。
        if let Some(next) = entry.node.next.take() {
            min_heap.push(Reverse(HeapEntry {
                list_index: entry.list_index,
                node: next,
            }));
        }
        values.push(entry.node.val);
    }

    ListNode::from_vec(values)
}
