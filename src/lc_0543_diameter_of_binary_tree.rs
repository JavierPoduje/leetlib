use crate::{Solution, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ans = walk(&root);
        ans.2
    }
}

// (branch_diameter, branches + current, max branches + current)
fn walk(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    if let Some(node) = root {
        let (left_height, left_diameter, left_max) = walk(&node.borrow().left);
        let (right_height, right_diameter, right_max) = walk(&node.borrow().right);

        let node_height = left_height.max(right_height) + 1;
        let node_diameter = left_height + right_height;

        (
            node_height,
            node_diameter,
            left_max.max(right_max.max(left_diameter.max(right_diameter.max(node_diameter)))),
        )
    } else {
        (0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let tree = TreeNode::from_vec(&vec![1, 2, 3, 4, 5]);
        println!("{}", tree.as_ref().unwrap().borrow());
        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn ex2_test() {
        let tree = TreeNode::from_vec(&vec![1, 2]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }

    //#[test]
    //fn ex3_test() {
    //    let tree = TreeNode::from_vec(&vec![
    //        4,
    //        -7,
    //        -3,
    //        i32::MIN,
    //        i32::MIN,
    //        -9,
    //        -3,
    //        9,
    //        -7,
    //        -4,
    //        i32::MIN,
    //        6,
    //        i32::MIN,
    //        -6,
    //        -6,
    //        i32::MIN,
    //        i32::MIN,
    //        0,
    //        6,
    //        5,
    //        i32::MIN,
    //        9,
    //        i32::MIN,
    //        i32::MIN,
    //        -1,
    //        -4,
    //        i32::MIN,
    //        i32::MIN,
    //        i32::MIN,
    //        -2,
    //    ]);
    //    assert_eq!(Solution::diameter_of_binary_tree(tree), 8);
    //}
}
