/* EASY
Given a set of points (x, y) on a 2D cartesian plane, find the two closest
points. For example, given the points [(1, 1), (-1, -1), (3, 4), (6, 1), (-1,
-6), (-4, -3)], return [(-1, -1), (1, 1)].
*/

use std::collections::hash_map::HashMap;

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
struct Point(i64, i64);

fn distance(p1: &Point, p2: &Point) -> f64 {
    let x = (p1.0 - p2.0).abs();
    let y = (p1.1 - p2.1).abs();
    ((x * x + y * y) as f64).sqrt()
}

fn problem_340(points: Vec<Point>) -> (Point, Point) {
    let mut distances = HashMap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = distance(&points[i], &points[j]);
            distances.insert((points[i], points[j]), dist);
        }
    }

    let mut min_tuple = *distances.iter().next().unwrap().0;
    let mut min_dist = *distances.iter().next().unwrap().1;

    for (tup, dist) in distances {
        if dist < min_dist {
            min_tuple = tup;
            min_dist = dist;
        }
    }

    min_tuple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_340() {
        assert_eq!(
            problem_340(vec![
                Point(1, 1),
                Point(-1, -1),
                Point(3, 4),
                Point(6, 1),
                Point(-1, -6),
                Point(-4, -3)
            ]),
            (Point(1, 1), Point(-1, -1))
        );
    }
}
