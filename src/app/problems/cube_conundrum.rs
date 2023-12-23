#[cfg(feature = "ssr")]
mod box_set;

#[cfg(feature = "ssr")]
use self::box_set::BoxSet;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => {
                let comparison = BoxSet {
                    red: 12,
                    green: 13,
                    blue: 14,
                };
                input.lines().filter_map(|line| {
                    let (game_descriptor, content) = line.split_once(": ").unwrap();
                    let game_id = game_descriptor.strip_prefix("Game ").unwrap().parse::<usize>().unwrap();
                    if content.split("; ").map(|x| -> BoxSet { x.into() }).all(|box_set| box_set <= comparison) {
                        Some(game_id)
                    } else {
                        None
                    }
                }).sum()
            },
            ProblemPart::Part2 => {
                input.lines().map(|line| {
                    let (_, content) = line.split_once(": ").unwrap();
                    let sum: BoxSet = content.split("; ").map(|x| -> BoxSet { x.into() }).sum();
                    sum.power()
                }).sum()
            }
        }
    }
}
