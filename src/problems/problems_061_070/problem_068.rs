/* MEDIUM
On our special chessboard, two bishops attack each other if they share the same
diagonal. This includes bishops that have another bishop located between them,
i.e. bishops can attack through pieces.

You are given N bishops, represented as (row, column) tuples on a M by M
chessboard. Write a function to count the number of pairs of bishops that attack
each other. The ordering of the pair doesn't matter: (1, 2) is considered the
same as (2, 1).

For example, given M = 5 and the list of bishops:

 * (0, 0)
 * (1, 2)
 * (2, 2)
 * (4, 0)

The board would look like this:

[b 0 0 0 0]
[0 0 b 0 0]
[0 0 b 0 0]
[0 0 0 0 0]
[b 0 0 0 0]

You should return 2, since bishops 1 and 3 attack each other, as well as bishops
3 and 4.
*/

fn two_bishops(bishops: Vec<(u32, u32)>) -> u32 {
    let mut num_diags = 0;

    for i in 0..bishops.len() {
        for j in i + 1..bishops.len() {
            let x_dif = bishops[i].0.abs_diff(bishops[j].0);
            let y_dif = bishops[i].1.abs_diff(bishops[j].1);
            if x_dif == y_dif {
                num_diags += 1;
            }
        }
    }

    num_diags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_bishops() {
        assert_eq!(two_bishops(vec![(0, 0), (1, 2), (2, 2), (4, 0)]), 2);
        assert_eq!(two_bishops(vec![(0, 0), (4, 4), (0, 4), (4, 0)]), 2);
        assert_eq!(two_bishops(vec![(0, 0), (4, 4), (0, 4), (4, 0), (2, 2)]), 6);
    }
}
