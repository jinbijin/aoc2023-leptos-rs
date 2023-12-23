#[cfg(feature = "ssr")]
mod hands;

#[cfg(feature = "ssr")]
use self::hands::{CamelBid, CamelCard, CamelJokerCard};

use crate::as_server_fn_with_timing;

#[cfg(feature = "ssr")]
fn solve_instance<T>(input: &str) -> usize
where
    T: Ord,
    CamelBid<T>: for<'a> From<&'a str>
{
    let mut bids = input.lines().map(|line| -> CamelBid<T> { line.into() }).collect::<Vec<CamelBid<T>>>();
    bids.sort_by(|x, y| x.hand.cmp(&y.hand));

    bids.into_iter().enumerate()
        .map(|(index, bid)| (index + 1) * bid.bid_size)
        .sum()
}

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => solve_instance::<CamelCard>(&input),
            ProblemPart::Part2 => solve_instance::<CamelJokerCard>(&input)
        }
    }
}
