/* EASY
Implement a stack that has the following methods:

 * push(val), which pushes an element onto the stack
 * pop(), which pops off and returns the topmost element of the stack. If there
   are no elements in the stack, then it should throw an error or return null.
 * max(), which returns the maximum value in the stack currently. If there are
   no elements in the stack, then it should throw an error or return null.

Each method should run in constant time.
*/

struct Stack<T> {
    values: Vec<T>,
}

impl<T: Clone + std::cmp::Ord> Stack<T> {
    fn new() -> Stack<T> { Stack { values: Vec::new() } }

    fn push(&mut self, val: T) { self.values.push(val); }

    fn pop(&mut self) -> Option<T> { self.values.pop() }

    fn max(&self) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }
        let mut max_value: T = self.values[0].clone();

        for val in self.values.iter() {
            max_value = max_value.max(val.clone());
        }

        Some(max_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_043() {
        let mut stack = Stack::new();
        stack.push(0);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.max(), Some(3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
        assert_eq!(stack.pop(), None);

        assert_eq!(stack.max(), None);
    }
}
