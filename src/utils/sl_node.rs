#[derive(PartialEq, Debug)]
pub struct SlNode<T> {
    pub val: T,
    pub next: Option<Box<SlNode<T>>>,
}

impl<T: std::fmt::Debug> std::fmt::Display for SlNode<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`
        if self.next.is_some() {
            write!(f, "{:?} -> {}", self.val, self.next.as_ref().unwrap())
        } else {
            write!(f, "{:?}", self.val)
        }
    }
}

impl<T: std::clone::Clone> SlNode<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    pub fn new_from_vec(vals: Vec<T>) -> Self {
        let mut root = Self {
            val: vals[0].clone(),
            next: None,
        };

        root.append(vals[1..].to_vec());

        root
    }

    pub fn append(&mut self, vals: Vec<T>) {
        for val in vals {
            self.push(val);
        }
    }

    pub fn push(&mut self, val: T) {
        if self.next.is_none() {
            self.next = Some(Box::new(Self::new(val)))
        } else {
            self.next.as_mut().unwrap().push(val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sl_node() {
        let mut root = SlNode::new(1);
        root.push(2);
        root.push(3);

        assert_eq!(
            root,
            SlNode {
                val: 1,
                next: Some(Box::new(SlNode {
                    val: 2,
                    next: Some(Box::new(SlNode { val: 3, next: None }))
                }))
            }
        )
    }

    #[test]
    fn test_sl_node_from_vec() {
        let root = SlNode::new_from_vec(vec![1, 2, 3]);

        assert_eq!(
            root,
            SlNode {
                val: 1,
                next: Some(Box::new(SlNode {
                    val: 2,
                    next: Some(Box::new(SlNode { val: 3, next: None })),
                })),
            }
        )
    }
}
