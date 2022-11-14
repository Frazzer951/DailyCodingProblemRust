/* EASY
Given a N by M matrix of numbers, print out the matrix in a clockwise spiral.

For example, given the following matrix:

[[1,  2,  3,  4,  5],
 [6,  7,  8,  9,  10],
 [11, 12, 13, 14, 15],
 [16, 17, 18, 19, 20]]


You should print out the following:

1
2
3
4
5
10
15
20
19
18
17
16
11
6
7
8
9
14
13
12
*/

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn spiralizer(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut dir = Dir::Right;
    let (mut x_min, mut x_max) = (0, nums[0].len());
    let (mut y_min, mut y_max) = (0, nums.len());
    let (mut cur_x, mut cur_y) = (0, 0);

    while x_min != x_max && y_min != y_max {
        result.push(nums[cur_y][cur_x]);

        match dir {
            Dir::Up => {
                if cur_y == 0 || cur_y - 1 < y_min {
                    dir = Dir::Right;
                    x_min += 1;
                    cur_x += 1;
                } else {
                    cur_y -= 1;
                }
            }

            Dir::Down => {
                if cur_y + 1 >= y_max {
                    dir = Dir::Left;
                    x_max -= 1;
                    cur_x -= 1;
                } else {
                    cur_y += 1;
                }
            }
            Dir::Left => {
                if cur_x == 0 || cur_x - 1 < x_min {
                    dir = Dir::Up;
                    y_max -= 1;
                    cur_y -= 1;
                } else {
                    cur_x -= 1;
                }
            }
            Dir::Right => {
                if cur_x + 1 >= x_max {
                    dir = Dir::Down;
                    y_min += 1;
                    cur_y += 1;
                } else {
                    cur_x += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiralizer() {
        assert_eq!(
            spiralizer(vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20]
            ]),
            vec![1, 2, 3, 4, 5, 10, 15, 20, 19, 18, 17, 16, 11, 6, 7, 8, 9, 14, 13, 12]
        );
    }
}
