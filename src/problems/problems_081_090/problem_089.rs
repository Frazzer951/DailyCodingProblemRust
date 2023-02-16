/* MEDIUM
Determine whether a tree is a valid binary search tree.

A binary search tree is a tree with two children, left and right, and satisfies
the constraint that the key in the left child must be less than or equal to the
root and the key in the right child must be greater than or equal to the root.
*/

use crate::utils::bt_node::BtNode;

fn valid_bst<T: PartialOrd>(root: &BtNode<T>) -> bool {
    if let Some(left) = &root.l {
        if root.val < left.val {
            return false;
        }
        if !valid_bst(left) {
            return false;
        }
    }

    if let Some(right) = &root.r {
        if root.val > right.val {
            return false;
        }
        if !valid_bst(right) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bst_1() {
        assert!(valid_bst(&BtNode {
            val: 1,
            l: Some(Box::new(BtNode::new(0))),
            r: Some(Box::new(BtNode::new(2))),
        }));
    }

    #[test]
    fn test_valid_bst_2() {
        assert!(!valid_bst(&BtNode {
            val: 1,
            l: Some(Box::new(BtNode::new(2))),
            r: Some(Box::new(BtNode::new(0))),
        }));
    }
}
