/* EASY
Suppose an arithmetic expression is given as a binary tree. Each leaf is an
integer and each internal node is one of '+', '−', '∗', or '/'.

Given the root to such a tree, write a function to evaluate it.

For example, given the following tree:

    *
   / \
  +    +
 / \  / \
3  2  4  5


You should return 45, as it is (3 + 2) * (4 + 5).
*/

use crate::utils::bt_node::BtNode;

fn problem_050(node: BtNode<char>) -> i64 {
    match node.val {
        '*' => {
            let left = problem_050(*node.l.unwrap());
            let right = problem_050(*node.r.unwrap());
            left * right
        }
        '/' => {
            let left = problem_050(*node.l.unwrap());
            let right = problem_050(*node.r.unwrap());
            left / right
        }
        '+' => {
            let left = problem_050(*node.l.unwrap());
            let right = problem_050(*node.r.unwrap());
            left + right
        }
        '-' => {
            let left = problem_050(*node.l.unwrap());
            let right = problem_050(*node.r.unwrap());
            left - right
        }
        x => x.to_string().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_050() {
        let root = BtNode {
            val: '*',
            l: Some(Box::new(BtNode {
                val: '+',
                l: Some(Box::new(BtNode {
                    val: '3',
                    l: None,
                    r: None,
                })),
                r: Some(Box::new(BtNode {
                    val: '2',
                    l: None,
                    r: None,
                })),
            })),
            r: Some(Box::new(BtNode {
                val: '+',
                l: Some(Box::new(BtNode {
                    val: '4',
                    l: None,
                    r: None,
                })),
                r: Some(Box::new(BtNode {
                    val: '5',
                    l: None,
                    r: None,
                })),
            })),
        };
        assert_eq!(problem_050(root), 45);
    }
}
