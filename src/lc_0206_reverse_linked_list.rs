use crate::ListNode;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = vec![];
        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let mut ptr = None;
        for val in arr {
            let mut node = ListNode::new(val);
            node.next = ptr;
            ptr = Some(Box::new(node));
        }

        ptr
    }
}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    use super::*;

    #[test]
    fn ex1_test() {
        let list = ListNode::from_vec(&vec![1, 2, 3, 4, 5]);
        let reverse = Solution::reverse_list(list);
        assert_eq!(reverse.unwrap().as_ref().into_array(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn ex2_test() {
        let list = ListNode::from_vec(&vec![1, 2]);
        let reverse = Solution::reverse_list(list);
        assert_eq!(reverse.unwrap().as_ref().into_array(), vec![2, 1]);
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
