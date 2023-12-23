#[cfg(feature = "ssr")]
mod timeline;

#[cfg(feature = "ssr")]
use self::timeline::{InstabilityTimeline, Timeline};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> isize {
        let timelines = input.lines()
            .map(|line| -> Timeline { line.into() })
            .map(|timeline| InstabilityTimeline::new(timeline));

        match part {
            ProblemPart::Part1 => timelines.map(|x| x.extrapolate()).sum(),
            ProblemPart::Part2 => timelines.map(|x| x.extrapolate_backwards()).sum()
        }
    }
}