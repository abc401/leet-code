use std::rc::Rc;
use std::cell::{RefCell, RefMut};

// Definitions for the parameter types
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // let Some(rc) = root else {
    //     return 0;
    // };
    match root {
        Some(rc) => {
            let treenode = rc.borrow_mut();
            return root_to_leaf_numbers(0, treenode).iter().sum();
        },
        None => {
            return 0;
        }
    }
}

fn root_to_leaf_numbers(starting_val: i32, root: RefMut<TreeNode>) -> Vec<i32> {
    let mut result = Vec::new();
    let new_starting_val = starting_val*10 + root.val;
    match (&root.left, &root.right) {
        (Some(left), Some(right)) => {
            result.append(&mut root_to_leaf_numbers(new_starting_val, left.borrow_mut()));
            result.append(&mut root_to_leaf_numbers(new_starting_val, right.borrow_mut()));
        },
        (Some(rc), None) | (None, Some(rc)) => {
            result.append(&mut root_to_leaf_numbers(new_starting_val, rc.borrow_mut()));
        }
        _ => {
            result.append(&mut Vec::from([new_starting_val]));
        }
    }
    return result;
}