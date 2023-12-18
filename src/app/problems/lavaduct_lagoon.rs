#[cfg(feature = "ssr")]
mod dig_plan;
#[cfg(feature = "ssr")]
mod lavaduct_terrain;

use leptos::*;
use super::{ProblemPart, TimedProblemForm, TimedSolutionResponse};
#[cfg(feature = "ssr")]
use super::with_timing;
#[cfg(feature = "ssr")]
use self::{dig_plan::DigPlan, lavaduct_terrain::TrenchEdge};

#[cfg(feature = "ssr")]
fn solve(part: ProblemPart, input: String) -> isize {
    let dig_plan = input.parse::<DigPlan>().unwrap();
    let trench_edge = match part {
        ProblemPart::Part1 => TrenchEdge::from(dig_plan.original_steps()),
        ProblemPart::Part2 => TrenchEdge::from(&dig_plan.new_steps())
    };

    trench_edge.area()
}

#[server(LavaductLagoon)]
pub async fn solve_timed(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    with_timing(&solve, part, input)
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<LavaductLagoon>();

    view! {
        <TimedProblemForm name="Day 18: Lavaduct Lagoon" action=action />
    }
}
