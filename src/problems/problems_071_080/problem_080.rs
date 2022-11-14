/* EASY
Given the root of a binary tree, return a deepest node. For example, in the
following tree, return d.

    a
   / \
  b   c
 /
d
*/

use crate::utils::bt_node::BtNode;

fn deepest_node_helper(root: &BtNode<i32>, depth: u32) -> (i32, u32) {
    if root.l.is_none() && root.r.is_none() {
        return (root.val, depth);
    }
    if root.l.is_some() && root.r.is_some() {
        let left = deepest_node_helper(root.l.as_ref().unwrap().as_ref(), depth + 1);
        let right = deepest_node_helper(root.r.as_ref().unwrap().as_ref(), depth + 1);
        if left.1 >= right.1 {
            return left;
        } else {
            return right;
        }
    }
    if root.l.is_some() {
        return deepest_node_helper(root.l.as_ref().unwrap().as_ref(), depth + 1);
    }
    deepest_node_helper(root.r.as_ref().unwrap().as_ref(), depth + 1)
}

fn deepest_node(root: &BtNode<i32>) -> i32 {
    deepest_node_helper(root, 0).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepest_node() {
        let root = BtNode {
            val: 1,
            l: Some(Box::new(BtNode {
                val: 2,
                l: Some(Box::new(BtNode {
                    val: 4,
                    l: None,
                    r: None,
                })),
                r: None,
            })),
            r: Some(Box::new(BtNode {
                val: 3,
                l: None,
                r: None,
            })),
        };
        assert_eq!(deepest_node(&root), 4);
    }
}
