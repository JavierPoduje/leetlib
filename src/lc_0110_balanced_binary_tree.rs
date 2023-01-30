use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

#[derive(Debug)]
struct Res {
    is_balanced: bool,
    height: i32,
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let ans = walk(&root);
    ans.is_balanced
}

fn walk(root: &Option<Rc<RefCell<TreeNode>>>) -> Res {
    if root.is_none() {
        return Res {
            is_balanced: true,
            height: 0,
        };
    }

    let left = walk(&root.as_ref().unwrap().borrow().left);
    let right = walk(&root.as_ref().unwrap().borrow().right);

    let is_balanced = if !left.is_balanced || !right.is_balanced {
        false
    } else {
        (left.height - right.height).abs() <= 1
    };

    Res {
        is_balanced,
        height: left.height.max(right.height) + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let tree = TreeNode::from_vec(&vec![3, 9, 20, i32::MIN, i32::MIN, 15, 7]);
        assert_eq!(is_balanced(tree), true);
    }

    #[test]
    fn ex2_test() {
        let tree = TreeNode::from_vec(&vec![1, 2, 2, 3, 3, i32::MIN, i32::MIN, 4, 4]);
        assert_eq!(is_balanced(tree), false);
    }
}
