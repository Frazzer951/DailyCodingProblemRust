/* HARD
Given an array of strictly the characters 'R', 'G', and 'B', segregate the
values of the array so that all the Rs come first, the Gs come second, and the
Bs come last. You can only swap elements of the array.

Do this in linear time and in-place.

For example, given the array ['G', 'B', 'R', 'R', 'B', 'R', 'G'], it should
become ['R', 'R', 'R', 'G', 'G', 'B', 'B'].
*/

fn problem_035(arr: &mut [char]) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for c in arr.iter() {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            'B' => b += 1,
            _ => {
                panic!("{} isn't R, G, or B", c)
            }
        }
    }
    g += r;
    b += g;

    for i in 0..arr.len() {
        match arr[i] {
            'R' => {
                arr.swap(i, r - 1);
                r -= 1;
            }
            'G' => {
                arr.swap(i, g - 1);
                g -= 1;
            }
            'B' => {
                arr.swap(i, b - 1);
                b -= 1;
            }
            _ => panic!("How did you get here?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_035() {
        let mut vec = vec!['G', 'B', 'R', 'R', 'B', 'R', 'G'];
        problem_035(&mut vec);
        assert_eq!(vec, vec!['R', 'R', 'R', 'G', 'G', 'B', 'B']);
    }
}
