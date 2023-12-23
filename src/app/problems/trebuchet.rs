#[cfg(feature = "ssr")]
mod number_map;

#[cfg(feature = "ssr")]
use number_map::create_number_map;

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
                let number_map = create_number_map();
                input.lines().map(|line| {
                    let first_digit = *number_map.iter()
                        .filter(|(key, _)| line.contains(**key))
                        .min_by(|(key_x, _), (key_y, _)| line.find(**key_x).unwrap().cmp(&line.find(**key_y).unwrap())).unwrap()
                        .1;
                    let last_digit = *number_map.iter()
                        .filter(|(key, _)| line.contains(**key))
                        .max_by(|(key_x, _), (key_y, _)| line.rfind(**key_x).unwrap().cmp(&line.rfind(**key_y).unwrap())).unwrap()
                        .1;
                    first_digit * 10 + last_digit
                }).sum()
            }
        }
    }
}
