#[cfg(feature = "ssr")]
mod race;

#[cfg(feature = "ssr")]
use self::race::{Race, RaceProgram};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => {
                let race_program: RaceProgram = (&*input).into();
                race_program.iter().map(|race| race.get_leniency()).product()
            },
            ProblemPart::Part2 => {
                let race: Race = (&*input).into();
                race.get_leniency()
            }
        }
    }
}
