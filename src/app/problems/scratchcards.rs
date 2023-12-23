#[cfg(feature = "ssr")]
mod cards;

#[cfg(feature = "ssr")]
use self::cards::{AsScratchcardCopy, Scratchcard};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => {
                input.lines()
                    .map(|x| -> Scratchcard { x.into() })
                    .map(|x| x.value())
                    .sum()
            },
            ProblemPart::Part2 => {
                input.lines()
                    .map(|x| -> Scratchcard { x.into() })
                    .process_copies()
                    .sum()
            }
        }
    }
}
