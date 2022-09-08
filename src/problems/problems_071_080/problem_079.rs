/* MEDIUM
Given an array of integers, write a function to determine whether the array
could become non-decreasing by modifying at most 1 element.

For example, given the array [10, 5, 7], you should return true, since we can
modify the 10 into a 1 to make the array non-decreasing.

Given the array [10, 5, 1], you should return false, since we can't modify any
one element to get a non-decreasing array.
*/

fn is_non_decreasing(arr: &Vec<i32>, skip: Option<usize>) -> bool {
    for i in 0..arr.len() - 1 {
        if let Some(skip) = skip {
            if i == skip {
                continue;
            }
            if i + 1 == skip {
                if i + 2 >= arr.len() {
                    continue;
                }
                if arr[i] > arr[i + 2] {
                    return false;
                }
            }
        }

        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

fn non_decreasing_removing_1(arr: Vec<i32>) -> bool {
    if is_non_decreasing(&arr, None) {
        return true;
    }

    for i in 0..arr.len() {
        if is_non_decreasing(&arr, Some(i)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_decreasing_removing_1() {
        assert!(non_decreasing_removing_1(vec![10, 5, 7]));
        assert!(!non_decreasing_removing_1(vec![10, 5, 1]));
    }
}
