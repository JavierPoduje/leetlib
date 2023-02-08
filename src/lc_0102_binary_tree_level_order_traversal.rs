use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut current_level_queue = VecDeque::new();
        current_level_queue.push_back(root.clone());

        let mut ans = Vec::new();

        while !current_level_queue.is_empty() {
            let mut next_level_queue = VecDeque::new();
            let mut current_level = Vec::new();

            while let Some(raw_node) = current_level_queue.pop_front() {
                let node = raw_node.unwrap();
                let left_node = node.clone().borrow().left.clone().to_owned();
                let right_node = node.borrow().right.clone();
                current_level.push(node.clone().borrow().val);

                if left_node.is_some() {
                    next_level_queue.push_back(left_node.clone());
                }
                if right_node.is_some() {
                    next_level_queue.push_back(right_node.clone());
                }
            }

            ans.push(current_level);

            current_level_queue = next_level_queue;
        }

        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1_test() {
        let tree = TreeNode::from_vec(&vec![3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        let ans = Solution::level_order(tree);
        assert_eq!(ans, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn ex2_test() {
        let tree = TreeNode::from_vec(&vec![1]);
        let ans = Solution::level_order(tree);
        assert_eq!(ans, vec![vec![1]]);
    }

    #[test]
    fn ex3_test() {
        let tree = TreeNode::from_vec(&vec![]);
        let ans = Solution::level_order(tree);
        assert_eq!(ans, Vec::<Vec<i32>>::new());
    }
}
