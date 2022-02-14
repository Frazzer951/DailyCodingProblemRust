/* MEDIUM
The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a
Monte Carlo method.

Hint: The basic equation of a circle is x2 + y2 = r2.
*/

#[allow(unused_macros)]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        let diff = ($x - $y).abs();
        if (diff >= $d) {
            panic!("{} - {} = {} which is greater than {}", $x, $y, diff, $d);
        }
    };
}

use rand::Rng;

fn monte_carlo_pi() -> f64 {
    let iterations = 100000;
    let mut rng = rand::thread_rng();

    let mut total = 0;
    let mut inside = 0;

    for _ in 0..iterations {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);

        if x * x + y * y <= 1.0 {
            inside += 1;
        }
        total += 1;
    }

    4.0 * (inside as f64 / total as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_014() {
        assert_delta!(monte_carlo_pi(), std::f64::consts::PI, 0.1);
    }
}
