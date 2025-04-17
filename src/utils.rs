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