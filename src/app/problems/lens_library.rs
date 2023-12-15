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
#[cfg(feature = "ssr")]
use super::with_timing;
use leptos::*;
use super::{ProblemPart, TimedProblemForm, TimedSolutionResponse};

#[cfg(feature = "ssr")]
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

#[server(LensLibrary)]
pub async fn solve_timed(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    with_timing(&solve, part, input)
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<LensLibrary>();

    view! {
        <TimedProblemForm name="Day 15: Lens Library" action=action />
    }
}
