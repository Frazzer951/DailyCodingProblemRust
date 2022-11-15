/* MEDIUM
Given three 32-bit integers x, y, and b, return x if b is 1 and y if b is 0,
using only mathematical or bit operations. You can assume b can only be 1 or 0.
*/

fn problem_085(x: i32, y: i32, b: i32) -> i32 {
    (x * b) | (y * (1 - b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_085() {
        assert_eq!(problem_085(1, 2, 1), 1);
        assert_eq!(problem_085(1, 2, 0), 2);
    }
}
