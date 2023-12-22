#[cfg(feature="ssr")]
mod grid_coords;
#[cfg(feature="ssr")]
mod grid_direction;
#[cfg(feature="ssr")]
mod grid;
#[cfg(feature="ssr")]
mod grid_loop;
#[cfg(feature="ssr")]
mod pipe;
#[cfg(feature="ssr")]
mod pipe_location;
#[cfg(feature="ssr")]
mod grid_region;

#[cfg(feature="ssr")]
use self::{grid::Grid, grid_loop::GridLoop, grid_region::{AsGridRegionIter, GridRegionType}, pipe::Pipe};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let pipe_grid: Vec<_> = input.lines()
            .map(|line| -> Vec<_> {
                line.chars().map(|c| -> Pipe { c.into() }).collect()
            })
            .collect();
        let grid: Grid = pipe_grid.into();
        let grid_loop: GridLoop = grid.loop_iter().collect();

        match part {
            ProblemPart::Part1 => grid_loop.diameter(),
            ProblemPart::Part2 => {
                grid.coords_iter()
                    .grid_region_iter(&grid_loop)
                    .filter(|x| { x.region_type == GridRegionType::Inside })
                    .count()
            }
        }
    }
}
