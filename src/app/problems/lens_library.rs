#[cfg(feature = "ssr")]
mod lava_hash;
#[cfg(feature = "ssr")]
mod lava_instruction;
#[cfg(feature = "ssr")]
mod lens_box;
#[cfg(feature = "ssr")]
mod focusing_power;

use leptos::*;
use super::{ProblemForm, ProblemPart};
#[cfg(feature = "ssr")]
use self::{focusing_power::FocusingPower, lava_hash::LavaHash, lava_instruction::LavaInstruction, lens_box::LensArray};

#[server(LensLibrary)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let instructions = input.split(',')
        .map(|part| -> LavaInstruction { part.into() });
    let result = match part {
        ProblemPart::Part1 => {
            instructions
                .map(|instruction| instruction.get_lava_value())
                .sum()
        },
        ProblemPart::Part2 => {
            instructions.collect::<LensArray>().get_focusing_power()
        }
    };

    Ok(format!("{}", result))
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<LensLibrary>();

    view! {
        <ProblemForm name="Day 14: Parabolic Reflector Dish" action=action />
    }
}
