use crate::{ListNode, Solution};

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        let mut flip_slow = false;
        while let Some(node) = fast {
            fast = &node.next;
            if flip_slow {
                slow = &slow.as_ref().unwrap().next;
            }
            flip_slow = !flip_slow;
        }

        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    use super::*;

    #[test]
    fn ex1_test() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let middle = Solution::middle_node(list).unwrap().into_array();
        assert_eq!(vec![3, 4, 5], middle);
    }

    #[test]
    fn ex2_test() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5, 6]);
        let middle = Solution::middle_node(list).unwrap().into_array();
        assert_eq!(vec![4, 5, 6], middle);
    }
}
