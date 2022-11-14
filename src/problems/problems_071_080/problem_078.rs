use crate::utils::sl_node::SlNode;

/* MEDIUM
Given k sorted singly linked lists, write a function to merge all the lists into
one sorted singly linked list.
*/

impl<T: PartialOrd + Copy> SlNode<T> {
    fn merge(&self, other: &Self) -> Self {
        let mut l1 = Some(self);
        let mut l2 = Some(other);

        let first_val = if self.val < other.val {
            let v = self.val;
            l1 = if self.next.is_some() {
                Some(self.next.as_ref().unwrap())
            } else {
                None
            };
            v
        } else {
            let v = other.val;
            l2 = if other.next.is_some() {
                Some(other.next.as_ref().unwrap())
            } else {
                None
            };
            v
        };

        let mut merged = SlNode::new(first_val);

        while let (Some(li1), Some(li2)) = (l1, l2) {
            let min_val = if li1.val < li2.val {
                let v = li1.val;
                l1 = if li1.next.is_some() {
                    Some(li1.next.as_ref().unwrap())
                } else {
                    None
                };
                v
            } else {
                let v = li2.val;
                l2 = if li2.next.is_some() {
                    Some(li2.next.as_ref().unwrap())
                } else {
                    None
                };
                v
            };
            merged.push(min_val);
        }

        if l1.is_some() {
            while let Some(l) = l1 {
                merged.push(l.val);
                l1 = if l.next.is_some() {
                    Some(l.next.as_ref().unwrap())
                } else {
                    None
                };
            }
        }

        if l2.is_some() {
            while let Some(l) = l2 {
                merged.push(l.val);
                l2 = if l.next.is_some() {
                    Some(l.next.as_ref().unwrap())
                } else {
                    None
                };
            }
        }

        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sl_merge() {
        let l1 = SlNode::new_from_vec(vec![2, 4, 6]);
        let l2 = SlNode::new_from_vec(vec![1, 3, 5]);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");

        let l1 = SlNode::new_from_vec(vec![1, 3, 5]);
        let l2 = SlNode::new_from_vec(vec![2, 4, 6]);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");
    }

    #[test]
    fn test_sl_merge_small() {
        let l1 = SlNode::new(1);
        let l2 = SlNode::new(2);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");

        let l1 = SlNode::new(2);
        let l2 = SlNode::new(1);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");
    }

    #[test]
    fn test_sl_merge_lobsided() {
        let l1 = SlNode::new_from_vec(vec![2, 3, 4]);
        let l2 = SlNode::new(1);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2, 3, 4]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");

        let l1 = SlNode::new(1);
        let l2 = SlNode::new_from_vec(vec![2, 3, 4]);

        let result = l1.merge(&l2);
        let expected = SlNode::new_from_vec(vec![1, 2, 3, 4]);

        assert_eq!(result, expected, "\npretty: {result} != {expected}");
    }
}
