/// 二分查找
pub fn binary_search<T: Ord>(items: &Vec<T>, target: &T, left: usize, right: usize) -> Option<usize> {
    if left > right {
        return None;
    }
    let mid = (left + right) / 2;
    if items[mid] < *target {
        return binary_search(items, target, mid+1, right);
    }
    if items[mid] > *target {
        return binary_search(items, target, left, mid-1);
    }
    return Some(mid);
}

pub fn quick_sort<T, F>(items: &mut [T], cmp: &F) 
where 
    F: Fn(&T, &T) -> bool,
{
    let n = items.len();
    if (n < 2) {return;}
    // pivot归位
    let mut j = 0;
    for i in 1..n {
        if cmp(&items[i], &items[0]) {
            j += 1;
            items.swap(i, j);
        }
    }
    items.swap(0, j);
    // 分片递归
    let (l, r) = items.split_at_mut(j);
    quick_sort(l, cmp);
    quick_sort(&mut r[1..], cmp);
}