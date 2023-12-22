#[cfg(feature = "ssr")]
mod lava_hash;
#[cfg(feature = "ssr")]
mod lava_instruction;
#[cfg(feature = "ssr")]
mod lens_box;
#[cfg(feature = "ssr")]
mod focusing_power;

#[cfg(feature = "ssr")]
use self::{focusing_power::FocusingPower, lava_hash::LavaHash, lava_instruction::LavaInstruction, lens_box::LensArray};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let instructions = input.split(',')
            .map(|part| -> LavaInstruction { part.into() });
        match part {
            ProblemPart::Part1 => {
                instructions
                    .map(|instruction| instruction.get_lava_value())
                    .sum()
            },
            ProblemPart::Part2 => {
                instructions.collect::<LensArray>().get_focusing_power()
            }
        }
    }
}
