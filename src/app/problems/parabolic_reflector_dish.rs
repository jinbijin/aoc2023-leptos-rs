#[cfg(feature = "ssr")]
mod platform_space;
#[cfg(feature = "ssr")]
mod platform;

#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use self::platform::Platform;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let platform: Platform = input.as_str().into();

        match part {
            ProblemPart::Part1 => {
                return platform.get_total_load();
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
                        return next_platform.get_total_load();
                    }

                    if let Some(previous_index) = previous_platforms.get(&next_platform) {
                        let previous_index = *previous_index;
                        let cycle_length = current_index - previous_index;
                        let cycles_to_go = (cycle_count - current_index) % cycle_length;
                        return total_loads[previous_index + cycles_to_go];
                    }

                    total_loads.push(next_platform.get_unshifted_load());
                    previous_platforms.insert(next_platform.clone(), current_index);
                    next_platform = next_platform.cycle();
                }
            }
        }
    }
}
