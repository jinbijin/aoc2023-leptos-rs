#[cfg(feature = "ssr")]
mod sand_slab;
#[cfg(feature = "ssr")]
mod sand_stack;

use leptos::*;
use super::{ProblemPart, TimedSolutionResponse, TimedProblemForm};

#[cfg(feature = "ssr")]
use super::with_timing;
#[cfg(feature = "ssr")]
use self::{sand_slab::SandSlabSnapshot, sand_stack::AsSandStack};

#[cfg(feature = "ssr")]
fn solve(part: ProblemPart, input: String) -> usize {
    let stack = input.parse::<SandSlabSnapshot>().unwrap()
        .as_sand_stack();
    match part {
        ProblemPart::Part1 => stack.disintegrateable_slab_count(),
        ProblemPart::Part2 => stack.load_bearing_score()
    }
}

#[server(SandSlabs)]
pub async fn solve_timed(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    with_timing(&solve, part, input)
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<SandSlabs>();

    view! {
        <TimedProblemForm name="Day 22: Sand Slabs" action=action />
    }
}
