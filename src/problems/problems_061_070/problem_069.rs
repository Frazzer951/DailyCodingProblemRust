/* EASY
Given a list of integers, return the largest product that can be made by
multiplying any three integers.

For example, if the list is [-10, -10, 5, 2], we should return 500, since that's
-10 * -10 * 5.

You can assume the list has at least three integers.
*/

fn largest_three_product(nums: Vec<i64>) -> i64 {
    if nums.len() < 3 {
        panic!("Input vector must have at least 3 integers")
    }
    let mut largest = nums[0] * nums[1] * nums[2];

    for i in 0..nums.len() - 2 {
        for j in i + 1..nums.len() - 1 {
            for k in j + 1..nums.len() {
                largest = largest.max(nums[i] * nums[j] * nums[k]);
            }
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_three_product() {
        assert_eq!(largest_three_product(vec![-10, -10, 5, 2]), 500);
    }
}
