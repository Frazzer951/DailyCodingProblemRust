/* MEDIUM
reduce (also known as fold) is a function that takes in an array, a combining
function, and an initial value and builds up a result by calling the combining
function on each element of the array, left to right. For example, we can write
sum() in terms of reduce:

def add(a, b):
    return a + b

def sum(lst):
    return reduce(lst, add, 0)


This should call add on the initial value with the first element of the array,
and then the result of that with the second element of the array, and so on
until we reach the end, when we return the sum of the array.

Implement your own version of reduce.
*/

fn add(a: i64, b: i64) -> i64 { a + b }

fn mult(a: i64, b: i64) -> i64 { a * b }

fn reduce(arr: Vec<i64>, func: fn(i64, i64) -> i64, init_val: i64) -> i64 {
    let mut val = init_val;

    for x in arr {
        val = func(val, x);
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_342() {
        assert_eq!(reduce(vec![1, 2, 3, 4], add, 0), 10);
        assert_eq!(reduce(vec![1, 2, 3, 4], mult, 1), 24);
    }
}
