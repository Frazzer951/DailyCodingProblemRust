/* MEDIUM
Implement a queue using two stacks. Recall that a queue is a FIFO (first-in,
first-out) data structure with the following methods: enqueue, which inserts an
element into the queue, and dequeue, which removes it.
*/

struct Queue {
    store: Vec<i32>,
    read: Vec<i32>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            store: vec![],
            read: vec![],
        }
    }

    pub fn enqueue(&mut self, num: i32) {
        self.store.push(num);
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.store.is_empty() {
            return None;
        }

        self.swap_stacks();
        let val = self.read.pop();
        self.swap_stacks();
        val
    }

    fn swap_stacks(&mut self) {
        if self.store.is_empty() {
            while !self.read.is_empty() {
                self.store.push(self.read.pop().unwrap())
            }
        } else {
            while !self.store.is_empty() {
                self.read.push(self.store.pop().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn test_queue_2() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));

        queue.enqueue(4);
        queue.enqueue(5);

        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));

        assert_eq!(queue.dequeue(), None);
    }
}
