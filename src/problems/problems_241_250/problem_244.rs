// NOT DONE

/* EASY
The Sieve of Eratosthenes is an algorithm used to generate all prime numbers
smaller than N. The method is to take increasingly larger prime numbers, and
mark their multiples as composite.

For example, to find all primes less than 100, we would first mark [4, 6, 8,
...] (multiples of two), then [6, 9, 12, ...] (multiples of three), and so on.
Once we have done this for all primes less than N, the unmarked numbers that
remain will be prime.

Implement this algorithm.

Bonus: Create a generator that produces primes indefinitely (that is, without
taking N as an input).
*/

fn problem_244() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_problem_244() {
        assert_eq!(problem_244(), 1);
    }
}
