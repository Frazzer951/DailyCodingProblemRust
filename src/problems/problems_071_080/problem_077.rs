/* EASY
Given a list of possibly overlapping intervals, return a new list of intervals
where all overlapping intervals have been merged.

The input list is not necessarily ordered in any way.

For example, given [(1, 3), (5, 8), (4, 10), (20, 25)], you should return [(1,
3), (4, 10), (20, 25)].
*/

use std::collections::vec_deque::VecDeque;

fn overlaps(i1: &(i32, i32), i2: &(i32, i32)) -> bool {
    if i1.0 >= i2.0 && i1.0 <= i2.1 {
        return true;
    }
    if i2.0 >= i1.0 && i2.0 <= i1.1 {
        return true;
    }

    false
}

fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut merged = vec![];
    let mut intervals = VecDeque::from(intervals);

    while !intervals.is_empty() {
        let mut cur_interval = intervals.pop_front().unwrap();
        intervals.retain(|interval| {
            if overlaps(&cur_interval, interval) {
                cur_interval.0 = cur_interval.0.min(interval.0);
                cur_interval.1 = cur_interval.1.max(interval.1);
                return false;
            }

            true
        });
        merged.push(cur_interval);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        assert_eq!(
            merge_intervals(vec![(1, 3), (5, 8), (4, 10), (20, 25)]),
            vec![(1, 3), (4, 10), (20, 25)]
        );
    }
}
