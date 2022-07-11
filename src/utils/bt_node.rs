#[derive(PartialEq, Debug)]
pub struct BtNode<T> {
    pub val: T,
    pub l: Option<Box<BtNode<T>>>,
    pub r: Option<Box<BtNode<T>>>,
}

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> BtNode<T> {
    pub fn new(val: T) -> BtNode<T> {
        BtNode {
            val,
            l: None,
            r: None,
        }
    }

    pub fn new_from_vec(vals: Vec<T>) -> BtNode<T> {
        let mut root = BtNode {
            val: vals[0].clone(),
            l: None,
            r: None,
        };

        root.push(vals[1..].to_vec());

        root
    }

    pub fn insert(&mut self, new_val: T) {
        if self.val == new_val {
            return;
        }
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match *target_node {
            Some(ref mut subnode) => subnode.insert(new_val),
            None => {
                let new_node = BtNode {
                    val: new_val,
                    l: None,
                    r: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    pub fn push(&mut self, vals: Vec<T>) {
        for val in vals {
            self.insert(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_node() {
        let mut x = BtNode {
            val: "m",
            l: None,
            r: None,
        };
        x.insert("z");
        x.insert("b");
        x.insert("c");
        assert_eq!(
            x,
            BtNode {
                val: "m",
                l: Some(Box::new(BtNode {
                    val: "b",
                    l: None,
                    r: Some(Box::new(BtNode {
                        val: "c",
                        l: None,
                        r: None,
                    })),
                })),
                r: Some(Box::new(BtNode {
                    val: "z",
                    l: None,
                    r: None,
                })),
            }
        );
    }

    #[test]
    fn test_bt_new() {
        let root = BtNode::new("a");
        assert_eq!(
            root,
            BtNode {
                val: "a",
                l: None,
                r: None,
            },
        );
    }

    #[test]
    fn test_bt_new_from_vec() {
        let root = BtNode::new_from_vec(vec!["b", "a", "c"]);
        assert_eq!(
            root,
            BtNode {
                val: "b",
                l: Some(Box::new(BtNode {
                    val: "a",
                    l: None,
                    r: None,
                })),
                r: Some(Box::new(BtNode {
                    val: "c",
                    l: None,
                    r: None,
                })),
            },
        );
    }
}
