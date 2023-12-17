#[cfg(feature = "ssr")]
mod gear_city_grid;
#[cfg(feature = "ssr")]
mod crucible_planner;

use leptos::*;
use super::{ProblemPart, TimedProblemForm, TimedSolutionResponse};

#[cfg(feature = "ssr")]
use super::with_timing;
#[cfg(feature = "ssr")]
use self::{crucible_planner::{CrucibleConfig, CruciblePlanner}, gear_city_grid::GearCityGrid};

#[cfg(feature = "ssr")]
fn solve(part: ProblemPart, input: String) -> usize {
    let grid = input.parse::<GearCityGrid>().unwrap();
    let config = match part {
        ProblemPart::Part1 => CrucibleConfig::legacy(),
        ProblemPart::Part2 => CrucibleConfig::ultra()
    };

    CruciblePlanner::from(&grid, config).minimum_heat_loss()
}

#[server(ClumsyCrucible)]
pub async fn solve_timed(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    with_timing(&solve, part, input)
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<ClumsyCrucible>();

    view! {
        <TimedProblemForm name="Day 17: Clumsy Crucible" action=action />
    }
}
