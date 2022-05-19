/* EASY
You run an e-commerce website and want to record the last N order ids in a log.
Implement a data structure to accomplish this, with the following API:

 * record(order_id): adds the order_id to the log
 * get_last(i): gets the ith last element from the log. i is guaranteed to be
   smaller than or equal to N.

You should be as efficient with time and space as possible.
*/

struct OrderLog {
    orders: Vec<i64>,
    cur_index: usize,
    size: usize,
}

impl OrderLog {
    fn new(n: usize) -> OrderLog {
        OrderLog { orders: vec![0; n],
                   cur_index: 0,
                   size: n }
    }

    fn record(&mut self, order_id: i64) {
        self.orders[self.cur_index] = order_id;
        self.cur_index = (self.cur_index + 1) % self.size;
    }

    fn get_last(&self, i: usize) -> i64 {
        let diff = self.cur_index as i64 - i as i64;
        let size = self.size as i64;
        let index = (((diff % size) + size) % size) as usize;
        self.orders[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_016() {
        let mut log = OrderLog::new(10);

        log.record(1);
        log.record(2);
        log.record(3);
        log.record(4);
        log.record(5);
        log.record(6);
        log.record(7);
        log.record(8);
        log.record(9);
        log.record(10);

        assert_eq!(log.get_last(1), 10);
        assert_eq!(log.get_last(2), 9);
        assert_eq!(log.get_last(3), 8);
        assert_eq!(log.get_last(4), 7);
        assert_eq!(log.get_last(5), 6);
        assert_eq!(log.get_last(6), 5);
        assert_eq!(log.get_last(7), 4);
        assert_eq!(log.get_last(8), 3);
        assert_eq!(log.get_last(9), 2);
        assert_eq!(log.get_last(10), 1);

        log.record(20);
        assert_eq!(log.get_last(1), 20);
        assert_eq!(log.get_last(10), 2);
    }
}
