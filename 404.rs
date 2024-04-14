// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, false)
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if is_left {
                        return node.borrow().val;
                    } else {
                        return 0;
                    };
                }
                let left: i32 = Self::dfs(&node.borrow().left, true);
                let right: i32 = Self::dfs(&node.borrow().right, false);

                left + right
            }
        }
    }
}
