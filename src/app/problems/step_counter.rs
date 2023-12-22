#[cfg(feature = "ssr")]
mod garden;
#[cfg(feature = "ssr")]
mod parallel_universe_walker;

use leptos::*;
use super::{ProblemPart, TimedSolutionResponse, TimedProblemForm};

#[cfg(feature = "ssr")]
use super::with_timing;
#[cfg(feature = "ssr")]
use self::{garden::Garden, parallel_universe_walker::AsParallelUniverseWalker};

// Assumptions that seem to hold for part 2:
// * grid is square and (3 mod 4) x (3 mod 4), and the start is in the dead center
// * outer columns and rows are empty
// * center column and row are empty
// * area around intersections of these columns and rows are clear of rocks
// * rocks are "sufficiently sparse" (no visitable squares are left behind after traveling around the edges)
#[cfg(feature = "ssr")]
fn solve(part: ProblemPart, input: String) -> usize {
    let garden = input.parse::<Garden>().unwrap();

    match part {
        ProblemPart::Part1 => {
            let walker = garden.as_parallel_universe_walker();

            walker.possible_step_count_after(64)
        },
        ProblemPart::Part2 => {
            let center_result = garden.as_parallel_universe_walker().fill_plots();

            let step_count = 26_501_365usize;

            let edge_length = garden.width();
            let shape = (step_count + 1) / edge_length;
            let remainder_cardinal = (step_count - ((edge_length + 1) / 2)) % edge_length;
            let remainder_diagonal = (step_count - (edge_length + 1)) % edge_length;

            let full_odd_count_root = ((shape - 1) / 2) * 2 + 1;
            let full_odd_count = full_odd_count_root * full_odd_count_root;
            let full_even_count_root = (shape / 2) * 2;
            let full_even_count = full_even_count_root * full_even_count_root;

            let full_contribution = full_odd_count * center_result.odd_distance_count + full_even_count * center_result.even_distance_count;
            let east_contribution = garden.as_parallel_universe_walker_from(garden.east_plot()).possible_step_count_after(remainder_cardinal);
            let south_contribution = garden.as_parallel_universe_walker_from(garden.south_plot()).possible_step_count_after(remainder_cardinal);
            let north_contribution = garden.as_parallel_universe_walker_from(garden.north_plot()).possible_step_count_after(remainder_cardinal);
            let west_contribution = garden.as_parallel_universe_walker_from(garden.west_plot()).possible_step_count_after(remainder_cardinal);

            let north_east_contribution_1 = garden.as_parallel_universe_walker_from(garden.north_east_plot()).possible_step_count_after(remainder_diagonal);
            let north_east_contribution_2 = garden.as_parallel_universe_walker_from(garden.north_east_plot()).possible_step_count_after(remainder_diagonal + edge_length);
            let north_west_contribution_1 = garden.as_parallel_universe_walker_from(garden.north_west_plot()).possible_step_count_after(remainder_diagonal);
            let north_west_contribution_2 = garden.as_parallel_universe_walker_from(garden.north_west_plot()).possible_step_count_after(remainder_diagonal + edge_length);
            let south_east_contribution_1 = garden.as_parallel_universe_walker_from(garden.south_east_plot()).possible_step_count_after(remainder_diagonal);
            let south_east_contribution_2 = garden.as_parallel_universe_walker_from(garden.south_east_plot()).possible_step_count_after(remainder_diagonal + edge_length);
            let south_west_contribution_1 = garden.as_parallel_universe_walker_from(garden.south_west_plot()).possible_step_count_after(remainder_diagonal);
            let south_west_contribution_2 = garden.as_parallel_universe_walker_from(garden.south_west_plot()).possible_step_count_after(remainder_diagonal + edge_length);

            full_contribution
                + east_contribution
                + south_contribution
                + west_contribution
                + north_contribution
                + north_east_contribution_1 * shape + north_east_contribution_2 * (shape - 1)
                + north_west_contribution_1 * shape + north_west_contribution_2 * (shape - 1)
                + south_east_contribution_1 * shape + south_east_contribution_2 * (shape - 1)
                + south_west_contribution_1 * shape + south_west_contribution_2 * (shape - 1)
        }
    }
}

#[server(StepCounter)]
pub async fn solve_timed(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    with_timing(&solve, part, input)
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<StepCounter>();

    view! {
        <TimedProblemForm name="Day 21: Step Counter" action=action />
    }
}
