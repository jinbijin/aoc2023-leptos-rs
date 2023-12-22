#[cfg(feature = "ssr")]
mod module;

#[cfg(feature = "ssr")]
use self::module::ModuleConfiguration;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let mut module_collection = ModuleConfiguration::from_str(&input).into_modules();

        match part {
            ProblemPart::Part1 => {
                let total = 1000usize;
                let mut high_pulse_count = 0usize;
                let mut low_pulse_count = 0usize;

                for i in 0..total {
                    let (high, low) = module_collection.click_button(i);
                    high_pulse_count += high;
                    low_pulse_count += low;
                }

                high_pulse_count * low_pulse_count
            },
            ProblemPart::Part2 => {
                let mut click_count = 0usize;

                loop {
                    click_count += 1;
                    module_collection.click_button(click_count);
                }
            }
        }
    }
}
