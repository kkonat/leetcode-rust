use leetcode_rust::helpers::linked_list::{to_list, ListNode};

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let node = Some(Box::new(ListNode::new(0)));
        node
    }
}

fn main() {
    println!("solution #2 Add two numbers");
    assert_eq!(
        Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
        to_list(vec![7, 0, 8])
    );
}
