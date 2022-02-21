/* MEDIUM
Given a binary search tree and a range [a, b] (inclusive), return the sum of the
elements of the binary search tree within the range.

For example, given the following tree:

    5
   / \
  3   8
 / \ / \
2  4 6  10


and the range [4, 9], return 23 (5 + 4 + 6 + 8).
*/

use crate::utils::bt_node::BtNode;

fn problem_343(root: BtNode<i64>, lower: i64, upper: i64) -> i64 {
    let mut sum = 0;

    if root.val >= lower && root.val <= upper {
        sum += root.val;
    }

    if root.val >= lower && root.l.is_some() {
        sum += problem_343(*root.l.unwrap(), lower, upper);
    }

    if root.val <= upper && root.r.is_some() {
        sum += problem_343(*root.r.unwrap(), lower, upper);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_343() {
        let root = BtNode {
            val: 5,
            l: Some(Box::new(BtNode {
                val: 3,
                l: Some(Box::new(BtNode {
                    val: 2,
                    l: None,
                    r: None,
                })),
                r: Some(Box::new(BtNode {
                    val: 4,
                    l: None,
                    r: None,
                })),
            })),
            r: Some(Box::new(BtNode {
                val: 8,
                l: Some(Box::new(BtNode {
                    val: 6,
                    l: None,
                    r: None,
                })),
                r: Some(Box::new(BtNode {
                    val: 10,
                    l: None,
                    r: None,
                })),
            })),
        };

        assert_eq!(problem_343(root, 4, 9), 23);
    }
}
