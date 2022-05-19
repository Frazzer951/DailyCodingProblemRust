#[derive(PartialEq, Debug)]
pub struct BtNode<T> {
    pub val: T,
    pub l: Option<Box<BtNode<T>>>,
    pub r: Option<Box<BtNode<T>>>,
}
impl<T: std::cmp::PartialEq + std::cmp::PartialOrd> BtNode<T> {
    pub fn insert(&mut self, new_val: T) {
        if self.val == new_val {
            return;
        }
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match *target_node {
            Some(ref mut subnode) => subnode.insert(new_val),
            None => {
                let new_node = BtNode { val: new_val,
                                        l: None,
                                        r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_node() {
        let mut x = BtNode { val: "m",
                             l: None,
                             r: None };
        x.insert("z");
        x.insert("b");
        x.insert("c");
        assert_eq!(x,
                   BtNode { val: "m",
                            l: Some(Box::new(BtNode { val: "b",
                                                      l: None,
                                                      r: Some(Box::new(BtNode { val: "c",
                                                                                l: None,
                                                                                r: None })) })),
                            r: Some(Box::new(BtNode { val: "z",
                                                      l: None,
                                                      r: None })) });
    }
}
