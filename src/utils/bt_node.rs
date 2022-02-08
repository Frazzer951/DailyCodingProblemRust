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
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
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
}
