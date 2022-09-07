/* MEDIUM
An sorted array of integers was rotated an unknown number of times.

Given such an array, find the index of the element in the array in faster than
linear time. If the element doesn't exist in the array, return null.

For example, given the array [13, 18, 25, 2, 8, 10] and the element 8, return 4
(the index of 8 in the array).

You can assume all the integers in the array are unique.
*/

fn shifted_array_search(arr: Vec<i32>, num: i32) -> Option<usize> {
    let mut i = arr.len() / 2;
    let mut dist = i / 2;

    loop {
        if arr[0] > arr[i] && arr[i - 1] > arr[i] {
            break;
        }
        if dist == 0 {
            break;
        }
        if arr[0] <= arr[i] {
            i += dist;
        } else if arr[i - 1] <= arr[i] {
            i -= dist;
        } else {
            break;
        }
        dist /= 2;
    }

    let mut low = i;
    let mut high = i - 1;
    dist = arr.len() / 2;

    loop {
        if dist == 0 {
            return None;
        }

        let guess_index = (low + dist) % arr.len();
        let guess = arr[guess_index];
        if guess == num {
            return Some(guess_index);
        }

        if guess < num {
            low = (low + dist) % arr.len();
        }
        if guess > num {
            high = (arr.len() + high - dist) % arr.len();
        }

        dist /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_058() {
        assert_eq!(shifted_array_search(vec![13, 18, 25, 2, 8, 10], 8), Some(4));
    }

    #[test]
    fn test_problem_058_2() {
        assert_eq!(shifted_array_search(vec![2, 8, 10, 13, 18, 25], 8), Some(1));
    }

    #[test]
    fn test_problem_058_3() {
        assert_eq!(shifted_array_search(vec![25, 2, 8, 10, 13, 18], 10), Some(3));
    }

    #[test]
    fn test_problem_058_4() {
        assert_eq!(shifted_array_search(vec![25, 2, 8, 10, 13, 18], 25), Some(0));
    }

    #[test]
    fn test_problem_058_missing() {
        assert_eq!(shifted_array_search(vec![13, 18, 25, 2, 8, 10], 0), None);
    }
}
