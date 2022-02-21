/* EASY
Given a array of numbers representing the stock prices of a company in
chronological order, write a function that calculates the maximum profit you
could have made from buying and selling that stock once. You must buy before you
can sell it.

For example, given [9, 11, 8, 5, 7, 10], you should return 5, since you could
buy the stock at 5 dollars and sell it at 10 dollars.
*/

fn problem_047(prices: Vec<i64>) -> i64 {
    let mut min = prices[0];
    let mut max = prices[0];
    let mut max_profit = 0;

    for x in prices {
        if x < min {
            min = x;
            max = x;
        }
        if x > max {
            max = x;
            max_profit = max_profit.max(max - min);
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_problem_047() {
        assert_eq!(problem_047(vec![9, 11, 8, 5, 7, 10]), 5);
    }
}
