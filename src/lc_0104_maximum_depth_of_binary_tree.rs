use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;
use crate::TreeNode;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        traverse(&root)
    }
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let left = traverse(&node.borrow().left);
        let right = traverse(&node.borrow().right);
        left.max(right) + 1
    } else {
        0
    }
}

//#[cfg(test)]
//mod tests {

//    use super::*;

//    #[test]
//    fn ex1_test() {
//        let tree = TreeNode::from_vec(&vec![3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
//        assert_eq!(Solution::max_depth(tree), 3);
//    }

//    #[test]
//    fn ex2_test() {
//        let tree = TreeNode::from_vec(&vec![1, i32::MIN, 2]);
//        assert_eq!(Solution::max_depth(tree), 2);
//    }
//}
