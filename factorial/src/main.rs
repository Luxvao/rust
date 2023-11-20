#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct Solution;

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1 = l1.unwrap();
        let l2 = l2.unwrap();

        let mut l1_vec: Vec<i32> = Vec::new();
        let mut l2_vec: Vec<i32> = Vec::new();

        let mut result: Vec<i32> = Vec::new();

        extract_to_vec(&mut l1_vec, l1);
        extract_to_vec(&mut l2_vec, l2);

        let mut l1_vec: Vec<i32> = l1_vec.iter().rev().map(|i| *i).collect();
        let mut l2_vec: Vec<i32> = l2_vec.iter().rev().map(|i| *i).collect();

        let len = l1_vec.len().max(l2_vec.len());

        let mut carry = 0;
        let mut l1_iter = l1_vec.iter();
        let mut l2_iter = l2_vec.iter();

        for _ in 0..len {
            let tmp = match l1_iter.next() {
                Some(num) => num,
                None => &0,
            } + match l2_iter.next() {
                Some(num) => num,
                None => &0,
            } + carry;

            if tmp >= 10 {
                carry = 1;
            }
            else {
                carry = 0;
            }

            result.push(tmp % 10);
        }

        if carry >= 1 {
            result.push(carry);
        }

        let mut result = result.iter().rev().map(|i| *i).collect::<Vec<i32>>();

        return create_linked_list_from_iter_over_vec(&mut result.iter());
    }
}

fn extract_to_vec(vec: &mut Vec<i32>, node: Box<ListNode>) {
    vec.push(node.val);
    
    if let Some(node) = node.next {
        extract_to_vec(vec, node);
    }

    return;
}

fn create_linked_list_from_iter_over_vec(iter: &mut std::slice::Iter<'_, i32>) -> Option<Box<ListNode>> {
    let val = iter.next();

    if let None = val {
        return None;
    }

    Some(
        Box::new(
            ListNode {
                val: *val.unwrap(),
                next: create_linked_list_from_iter_over_vec(iter),
            }
        )
    )
}

fn main() {
    let mut x: f64 = 2.0;
    let n: i32 = -2;

    if n.is_negative() {
        let n = n.abs();

        for _ in 0..n+1 {
            x = x / x;
        }
    }

    println!("{}", x);
}
