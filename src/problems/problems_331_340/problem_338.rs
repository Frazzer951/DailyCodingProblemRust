/* MEDIUM
Given an integer n, find the next biggest integer with the same number of 1-bits
on. For example, given the number 6 (0110 in binary), return 9 (1001).
*/

fn next_biggest(mut num: i64) -> i64 {
    if num == 0 {
        return 0;
    }

    let mut count = 0;
    let mut mask = num & !(num - 1);
    while !((num & mask) > 0 && (num & (mask << 1)) == 0) {
        num &= !mask;
        count += 1;
        mask = num & !(num - 1);
    }

    num &= !mask;
    num |= mask << 1;

    for i in 0..count {
        num |= 1 << i;
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_biggest() {
        assert_eq!(next_biggest(6), 9);
        assert_eq!(next_biggest(0), 0);
    }
}
