#[cfg(feature = "ssr")]
mod platform_space;
#[cfg(feature = "ssr")]
mod platform;

use leptos::*;
use super::{ProblemForm, ProblemPart};

#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use self::platform::Platform;

#[server(ParabolicReflectorDish)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let platform: Platform = input.as_str().into();

    match part {
        ProblemPart::Part1 => {
            let result = platform.get_total_load();
            Ok(format!("{}", result))
        },
        ProblemPart::Part2 => {
            let cycle_count = 1_000_000_000usize;
            let mut total_loads: Vec<usize> = Vec::new();
            let mut previous_platforms: HashMap<Platform, usize> = HashMap::new();
            total_loads.push(platform.get_unshifted_load());
            previous_platforms.insert(platform.clone(), 0);

            let mut next_platform = platform.cycle();
            loop {
                let current_index = previous_platforms.len();

                if current_index == cycle_count {
                    let result = next_platform.get_total_load();

                    return Ok(format!("{}", result))
                }

                if let Some(previous_index) = previous_platforms.get(&next_platform) {
                    let previous_index = *previous_index;
                    let cycle_length = current_index - previous_index;
                    let cycles_to_go = (cycle_count - current_index) % cycle_length;
                    let result = total_loads[previous_index + cycles_to_go];

                    return Ok(format!("{}", result))
                }

                total_loads.push(next_platform.get_unshifted_load());
                previous_platforms.insert(next_platform.clone(), current_index);
                next_platform = next_platform.cycle();
            }
        }
    }
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<ParabolicReflectorDish>();

    view! {
        <ProblemForm name="Day 14: Parabolic Reflector Dish" action=action />
    }
}
