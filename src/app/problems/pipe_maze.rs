mod grid_coords;
mod grid_direction;
mod grid;
mod grid_loop;
mod pipe;
mod pipe_location;
mod grid_region;

use leptos::*;
use super::{ProblemPart, ProblemForm};
use self::{grid::Grid, grid_loop::GridLoop, grid_region::{AsGridRegionIter, GridRegionType}, pipe::Pipe};

#[server(PipeMaze)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let pipe_grid: Vec<_> = input.lines()
        .map(|line| -> Vec<_> {
            line.chars().map(|c| -> Pipe { c.into() }).collect()
        })
        .collect();
    let grid: Grid = pipe_grid.into();
    let grid_loop: GridLoop = grid.loop_iter().collect();

    match part {
        ProblemPart::Part1 => Ok(format!("{}", grid_loop.diameter())),
        ProblemPart::Part2 => {
            let count = grid.coords_iter()
                .grid_region_iter(&grid_loop)
                .filter(|x| { x.region_type == GridRegionType::Inside })
                .count();
            Ok(format!("{}", count))
        }
    }
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<PipeMaze>();

    view! {
        <ProblemForm name="Day 10: PipeMaze" action=action />
    }
}
