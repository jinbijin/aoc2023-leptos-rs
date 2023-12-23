#[cfg(feature = "ssr")]
mod almanac;

#[cfg(feature = "ssr")]
use self::almanac::{ Almanac, NumberRange, Resource, ResourceRange };

use crate::as_server_fn_with_timing;

#[cfg(feature = "ssr")]
pub fn solve_1(input: &str) -> usize {
    let (seeds_line, almanac_text) = input.split_once("\n\n").unwrap();
    let seeds = seeds_line
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| Resource { name: "seed".to_string(), number: x })
        .collect::<Vec<Resource>>();

    let almanac: Almanac = almanac_text.into();
    let mut locations: Vec<usize> = Vec::new();
    for resource in seeds.iter() {
        let mut resource = resource.clone();
        while &resource.name != "location" {
            resource = almanac.map(&resource)
        }

        locations.push(resource.number);
    }

    locations.into_iter().min().unwrap()
}

#[cfg(feature = "ssr")]
pub fn solve_2(input: &str) -> usize {
    let (seeds_line, almanac_text) = input.split_once("\n\n").unwrap();
    let seeds: Vec<ResourceRange> = read_ranges(seeds_line
        .strip_prefix("seeds: ").unwrap())
        .into_iter()
        .map(|x| ResourceRange { name: "seed".to_string(), number_range: x.clone() })
        .collect::<Vec<ResourceRange>>();

    let almanac: Almanac = almanac_text.into();
    let mut locations: Vec<usize> = Vec::new();
    for resource_range in seeds.iter() {
        let mut resource_ranges = vec![resource_range.clone()];
        while &resource_ranges[0].name != "location" {
            resource_ranges = resource_ranges.iter()
                .flat_map(|resource_range| almanac.map_range(resource_range))
                .collect();
        }

        for resource_range in resource_ranges {
            locations.push(resource_range.number_range.start);
        }
    }

    locations.into_iter().min().unwrap()
}

#[cfg(feature = "ssr")]
fn read_ranges(input: &str) -> Vec<NumberRange> {
    let mut parts = input.split(' ').map(|x| x.parse::<usize>().unwrap());
    let mut ranges = Vec::new();

    while let Some(first) = parts.next() {
        let second = parts.next().unwrap();

        ranges.push(NumberRange { start: first, length: second })
    }

    ranges
}

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        match part {
            ProblemPart::Part1 => solve_1(&input),
            ProblemPart::Part2 => solve_2(&input)
        }
    }
}
