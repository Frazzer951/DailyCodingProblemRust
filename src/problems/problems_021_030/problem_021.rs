/* EASY
Given an array of time intervals (start, end) for classroom lectures (possibly
overlapping), find the minimum number of rooms required.

For example, given [(30, 75), (0, 50), (60, 150)], you should return 2.
*/

fn problem_021(schedule: Vec<(u64, u64)>) -> u64 {
    if schedule.len() == 0 {
        return 0;
    }
    let mut conflicts = vec![];

    let cur_class = schedule[0];

    for i in 1..schedule.len() {
        let next_class = schedule[i];
        if (cur_class.1 >= next_class.0 && cur_class.0 < next_class.1)
            || (cur_class.1 > next_class.0 && cur_class.1 <= next_class.1)
            || (cur_class.0 < next_class.1 && cur_class.0 > next_class.0)
        {
            conflicts.push(schedule[i]);
        }
    }

    1 + problem_021(conflicts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_021() {
        assert_eq!(problem_021(vec![(30, 75), (0, 50), (60, 150)]), 2);
    }
}
