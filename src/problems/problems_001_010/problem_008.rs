/* EASY
A unival tree (which stands for "universal value") is a tree where all nodes
under it have the same value.

Given the root to a binary tree, count the number of unival subtrees.

For example, the following tree has 5 unival subtrees:

   0
  / \
 1   0
    / \
   1   0
  / \
 1   1
*/

use crate::utils::bt_node::BtNode;

fn same_children<T: std::cmp::PartialEq>(root: &BtNode<T>) -> bool {
    if root.l.is_none() && root.r.is_none() {
        return true;
    }

    if root.l.is_some() && root.val != root.l.as_ref().unwrap().val {
        return false;
    }
    if root.r.is_some() && root.val != root.r.as_ref().unwrap().val {
        return false;
    }

    let left = if root.l.is_some() {
        same_children(&*root.l.as_ref().unwrap())
    } else {
        true
    };
    let right = if root.r.is_some() {
        same_children(&*root.r.as_ref().unwrap())
    } else {
        true
    };

    left && right
}

fn count_unival_tree<T: std::cmp::PartialEq>(root: &BtNode<T>) -> i64 {
    if root.l.is_none() && root.r.is_none() {
        return 1;
    }
    let mut count = 0;

    if same_children(root) {
        count += 1;
    }
    let left = if root.l.is_some() {
        count_unival_tree(&*root.l.as_ref().unwrap())
    } else {
        0
    };
    let right = if root.r.is_some() {
        count_unival_tree(&*root.r.as_ref().unwrap())
    } else {
        0
    };
    count + left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_008() {
        /*
          0
         / \
        1   0
           / \
          1   0
         / \
        1   1
        */
        let node = BtNode {
            val: 0,
            l:   Some(Box::new(BtNode {
                val: 1,
                l:   None,
                r:   None,
            })),
            r:   Some(Box::new(BtNode {
                val: 0,
                l:   Some(Box::new(BtNode {
                    val: 1,
                    l:   Some(Box::new(BtNode {
                        val: 1,
                        l:   None,
                        r:   None,
                    })),
                    r:   Some(Box::new(BtNode {
                        val: 1,
                        l:   None,
                        r:   None,
                    })),
                })),
                r:   Some(Box::new(BtNode {
                    val: 0,
                    l:   None,
                    r:   None,
                })),
            })),
        };
        assert_eq!(count_unival_tree(&node), 5);
    }
}
