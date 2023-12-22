#[cfg(feature = "ssr")]
mod gear_city_grid;
#[cfg(feature = "ssr")]
mod crucible_planner;

#[cfg(feature = "ssr")]
use self::{crucible_planner::{CrucibleConfig, CruciblePlanner}, gear_city_grid::GearCityGrid};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let grid = input.parse::<GearCityGrid>().unwrap();
        let config = match part {
            ProblemPart::Part1 => CrucibleConfig::legacy(),
            ProblemPart::Part2 => CrucibleConfig::ultra()
        };

        CruciblePlanner::from(&grid, config).minimum_heat_loss()
    }
}
