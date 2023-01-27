#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let l1 = Solution::from(vec![2, 4, 3]);
    let l2 = Solution::from(vec![5, 6, 4]);
    println!("{:?}", Solution::add_two_numbers(l1, l2));
    let l1 = Solution::from(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = Solution::from(vec![9, 9, 9, 9]);
    println!("{:?}", Solution::add_two_numbers(l1, l2));
    let l1 = Solution::from(vec![0]);
    let l2 = Solution::from(vec![0]);
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current_l1 = &l1;
        let mut current_l2 = &l2;
        let mut result: Vec<i32> = Vec::new();
        let mut change = 0;
        while let Some(la) = current_l1 {
            if let Some(lb) = current_l2 {
                let current_val = la.val + lb.val + change;
                let rest = current_val % 10;
                change = current_val / 10;
                result.push(rest);
                current_l1 = &la.next;
                current_l2 = &lb.next;
            } else {
                let current_val = la.val + change;
                let rest = current_val % 10;
                change = current_val / 10;
                result.push(rest);
                current_l1 = &la.next;
            }
        }
        while let Some(lb) = current_l2 {
            let current_val = lb.val + change;
            let rest = current_val % 10;
            change = current_val / 10;
            result.push(rest);
            current_l2 = &lb.next;
        }
        while change > 0 {
            let current_val = change;
            let rest = current_val % 10;
            change = current_val / 10;
            result.push(rest);
        }
        Self::from(result)
    }
    pub fn from(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let node = Box::new(ListNode { val, next: current });
            current = Some(node);
        }
        current
    }
}
