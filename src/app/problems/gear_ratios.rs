#[cfg(feature = "ssr")]
mod gears;

#[cfg(feature = "ssr")]
use self::gears::read_gear_schematic;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => read_gear_schematic(&input)
                .get_parts_adjacent_to_symbol().into_iter()
                .map(|item| item.value)
                .sum(),
            ProblemPart::Part2 => read_gear_schematic(&input).get_gears()
        }
    }
}
