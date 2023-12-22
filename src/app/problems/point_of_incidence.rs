#[cfg(feature = "ssr")]
mod mirror;
#[cfg(feature = "ssr")]
mod volcanic_land;
#[cfg(feature = "ssr")]
mod volcanic_patch;

#[cfg(feature = "ssr")]
use self::volcanic_land::VolcanicLand;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let lands = input.split("\n\n")
            .map(|land_str| -> VolcanicLand { land_str.into() });

        match part {
            ProblemPart::Part1 =>
                lands.map(|land| land.find_mirror())
                    .map(|mirror| mirror.weight())
                    .sum(),
            ProblemPart::Part2 =>
                lands.map(|land| land.find_smudged_mirror())
                    .map(|mirror| mirror.weight())
                    .sum(),
        }
    }
}
