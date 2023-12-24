#[cfg(feature = "ssr")]
mod hailstone;
#[cfg(feature = "ssr")]
mod test_area;

#[cfg(feature = "ssr")]
use self::{hailstone::HailstoneCollection, test_area::{AreaRange, FlatArea}};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let hailstones = input.parse::<HailstoneCollection>().unwrap();
        match part {
            ProblemPart::Part1 => {
                let range_start = 200_000_000_000_000i128;
                let range_end = 400_000_000_000_000i128;
                let area = FlatArea {
                    x: AreaRange { start: range_start, end: range_end },
                    y: AreaRange { start: range_start, end: range_end },
                };

                hailstones.intersection_count_inside(&area)
            },
            ProblemPart::Part2 => {
                hailstones.print_equations();
                // ...to console, then plug this into Sage or whatever

                0
            }
        }
    }
}