#[cfg(feature = "ssr")]
mod sand_slab;
#[cfg(feature = "ssr")]
mod sand_stack;

#[cfg(feature = "ssr")]
use self::{sand_slab::SandSlabSnapshot, sand_stack::AsSandStack};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let stack = input.parse::<SandSlabSnapshot>().unwrap()
            .as_sand_stack();
        match part {
            ProblemPart::Part1 => stack.disintegrateable_slab_count(),
            ProblemPart::Part2 => stack.load_bearing_score()
        }
    }
}