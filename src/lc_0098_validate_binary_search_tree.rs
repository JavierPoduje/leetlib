use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        walk(&root, None, None)
    }
}

fn walk(
    root: &Option<Rc<RefCell<TreeNode>>>,
    left_boundary: Option<i32>,
    right_boundary: Option<i32>,
) -> bool {
    if root.is_none() {
        return true;
    }

    let tree = root.as_ref().unwrap();

    let valid_left_boundary = match left_boundary {
        Some(boundary) => boundary < tree.borrow().val,
        _ => true,
    };
    let valid_right_boundary = match right_boundary {
        Some(boundary) => boundary > tree.borrow().val,
        _ => true,
    };

    if !valid_left_boundary || !valid_right_boundary {
        return false;
    }

    let left = walk(&tree.borrow().left, left_boundary, Some(tree.borrow().val));
    let right = walk(
        &tree.borrow().right,
        Some(tree.borrow().val),
        right_boundary,
    );

    return left && right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert!(Solution::is_valid_bst(TreeNode::from_vec(&vec![2, 1, 3])));
    }

    #[test]
    fn ex2_test() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(&vec![5, 1, 4, i32::MIN, i32::MIN, 3, 6])),
            false
        );
    }

    #[test]
    fn ex3_test() {
        assert!(Solution::is_valid_bst(TreeNode::from_vec(&vec![
            2147483647
        ])));
    }

    #[test]
    fn ex4_test() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(&vec![2, 2, 2])),
            false
        );
    }
}
