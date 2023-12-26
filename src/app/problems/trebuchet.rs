#[cfg(feature = "ssr")]
mod trebuchet_state;

#[cfg(feature = "ssr")]
use trebuchet_state::read_line_value;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => {
                input.lines().map(|line| {
                    let first_digit = line.chars().find(|char| char.is_digit(10)).unwrap();
                    let last_digit = line.chars().rfind(|char| char.is_digit(10)).unwrap();
                    let number_string = format!("{}{}", first_digit, last_digit);
                    number_string.parse::<usize>().unwrap()
                }).sum()
            },
            ProblemPart::Part2 => {
                input.lines().map(read_line_value).sum()
            }
        }
    }
}
