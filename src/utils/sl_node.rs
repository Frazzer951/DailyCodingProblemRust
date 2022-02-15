#[derive(PartialEq, Debug)]
pub struct SlNode<T> {
    pub val: T,
    pub next: Option<Box<SlNode<T>>>,
}
impl<T: std::cmp::PartialEq + std::cmp::PartialOrd> SlNode<T> {
    pub fn insert(&mut self, new_val: T) {
        let target_node = &mut self.next;
        match *target_node {
            Some(ref mut subnode) => subnode.insert(new_val),
            None => {
                let new_node = SlNode {
                    val: new_val,
                    next: None,
                };
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
    fn test_sl_node() {
        let mut x = SlNode { val: "m", next: None };
        x.insert("z");
        x.insert("b");
        x.insert("c");
        assert_eq!(
            x,
            SlNode {
                val: "m",
                next: Some(Box::new(SlNode {
                    val: "z",
                    next: Some(Box::new(SlNode {
                        val: "b",
                        next: Some(Box::new(SlNode { val: "c", next: None }))
                    }))
                }))
            }
        );
    }
}
