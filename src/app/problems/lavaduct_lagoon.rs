#[cfg(feature = "ssr")]
mod dig_plan;
#[cfg(feature = "ssr")]
mod lavaduct_terrain;

#[cfg(feature = "ssr")]
use self::{dig_plan::DigPlan, lavaduct_terrain::TrenchEdge};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> isize {
        let dig_plan = input.parse::<DigPlan>().unwrap();
        let trench_edge = match part {
            ProblemPart::Part1 => TrenchEdge::from(dig_plan.original_steps()),
            ProblemPart::Part2 => TrenchEdge::from(&dig_plan.new_steps())
        };

        trench_edge.area()
    }
}
