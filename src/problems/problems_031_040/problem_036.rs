/* MEDIUM
Given the root to a binary search tree, find the second largest node in the
tree.
*/

use crate::utils::bt_node::BtNode;

fn second_largest_helper(root: &BtNode<i32>, c: &mut i32) -> Option<i32> {
    if root.r.is_some() {
        let result = second_largest_helper(root.r.as_ref().unwrap().as_ref(), c);
        if result.is_some() {
            return result;
        }
    }

    *c += 1;

    if *c == 2 {
        return Some(root.val);
    }

    if root.l.is_some() {
        let result = second_largest_helper(root.l.as_ref().unwrap().as_ref(), c);
        if result.is_some() {
            return result;
        }
    }

    None
}

fn bst_second_largest(node: BtNode<i32>) -> i32 {
    second_largest_helper(&node, &mut 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_036_1() {
        assert_eq!(
            bst_second_largest(BtNode::new_from_vec(vec![1, 2, 3, 4])),
            3
        );
    }

    #[test]
    fn test_problem_036_2() {
        assert_eq!(
            bst_second_largest(BtNode::new_from_vec(vec![4, 3, 2, 1])),
            3
        );
    }

    #[test]
    fn test_problem_036_3() {
        assert_eq!(
            bst_second_largest(BtNode::new_from_vec(vec![3, 2, 1, 4])),
            3
        );
    }
}
