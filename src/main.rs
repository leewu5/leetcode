use leetcode::solutions::p0002_add_two_numbers::{display, ListNode};

fn main() {
    let a = Some("1".to_string());
    let b = &a;
    // println!("{}", a.unwrap()); 
    println!("{}", b.as_ref().unwrap());
}
