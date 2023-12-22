#[cfg(feature = "ssr")]
mod cosmic_coords;
#[cfg(feature = "ssr")]
mod cosmic_grid;
#[cfg(feature = "ssr")]
mod cosmic_interval;

#[cfg(feature = "ssr")]
use self::cosmic_grid::CosmicGrid;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let cosmic_grid: CosmicGrid = input.as_str().into();
        let expansion_factor = match part {
            ProblemPart::Part1 => 2usize,
            ProblemPart::Part2 => 1000000usize
        };

        cosmic_grid.weight(expansion_factor)
    }
}
