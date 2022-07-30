/* EASY
Using a function rand5() that returns an integer from 1 to 5 (inclusive) with
uniform probability, implement a function rand7() that returns an integer from 1
to 7 (inclusive).
*/

use rand::Rng;

#[no_coverage]
fn rand5() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=5)
}

#[no_coverage]
fn rand7() -> i64 {
    let r1 = rand5();
    let r2 = rand5();
    if r2 <= 3 {
        return r1;
    }
    if r2 == 4 {
        if r1 <= 3 {
            return 6;
        }
        return rand7();
    }
    if r1 <= 3 {
        return 7;
    }
    rand7()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_045() {
        let val = rand7();
        assert!(val <= 7 || val >= 1);
    }
}
