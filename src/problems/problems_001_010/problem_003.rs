/* MEDIUM
Given the root to a binary tree, implement serialize(root), which serializes the
tree into a string, and deserialize(s), which deserializes the string back into
the tree.

For example, given the following Node class

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


The following test should pass:

node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
*/

use crate::utils::bt_node::BtNode;

fn serialize<T: std::fmt::Display>(root: &BtNode<T>) -> String {
    let mut node_str = root.val.to_string() + ":{";
    if root.l.is_some() {
        node_str += &serialize(&*root.l.as_ref().unwrap());
    }
    if root.r.is_some() {
        node_str += ",";
        node_str += &serialize(&*root.r.as_ref().unwrap());
    }
    node_str += "}";

    node_str
}

fn deserialize(node_str: String) -> BtNode<String> {
    let colon_index = node_str.find(':').unwrap_or(0);
    let node_name = String::from(&node_str[..colon_index]);
    let nodes = String::from(&node_str[colon_index + 1..]);

    if nodes.len() == 2 {
        return BtNode {
            val: node_name,
            l: None,
            r: None,
        };
    }

    let comma_index = nodes.find(',').unwrap_or_else(|| nodes.len());
    if comma_index == nodes.len() {
        let left = deserialize(String::from(&nodes[1..nodes.len() - 1]));
        return BtNode {
            val: node_name,
            l: Some(Box::new(left)),
            r: None,
        };
    }

    let left = deserialize(String::from(&nodes[1..comma_index]));
    let right = deserialize(String::from(&nodes[comma_index + 1..nodes.len() - 1]));

    BtNode {
        val: node_name,
        l: Some(Box::new(left)),
        r: Some(Box::new(right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_003() {
        let node = BtNode {
            val: String::from("root"),
            l: Some(Box::new(BtNode {
                val: String::from("left"),
                l: Some(Box::new(BtNode {
                    val: String::from("left.left"),
                    l: None,
                    r: None,
                })),
                r: None,
            })),
            r: Some(Box::new(BtNode {
                val: String::from("right"),
                l: None,
                r: None,
            })),
        };
        assert_eq!(serialize(&node), "root:{left:{left.left:{}},right:{}}");
    }

    #[test]
    fn test_deserialize_003() {
        let node = BtNode {
            val: String::from("root"),
            l: Some(Box::new(BtNode {
                val: String::from("left"),
                l: Some(Box::new(BtNode {
                    val: String::from("left.left"),
                    l: None,
                    r: None,
                })),
                r: None,
            })),
            r: Some(Box::new(BtNode {
                val: String::from("right"),
                l: None,
                r: None,
            })),
        };
        assert_eq!(deserialize(serialize(&node)), node);
    }

    #[test]
    fn test_serialize_deserialize_003() {
        // node = Node('root', Node('left', Node('left.left')), Node('right'))
        // assert deserialize(serialize(node)).left.left.val == 'left.left'
        let node = BtNode {
            val: String::from("root"),
            l: Some(Box::new(BtNode {
                val: String::from("left"),
                l: Some(Box::new(BtNode {
                    val: String::from("left.left"),
                    l: None,
                    r: None,
                })),
                r: None,
            })),
            r: Some(Box::new(BtNode {
                val: String::from("right"),
                l: None,
                r: None,
            })),
        };
        assert_eq!(deserialize(serialize(&node)).l.unwrap().l.unwrap().val, "left.left");
    }
}
