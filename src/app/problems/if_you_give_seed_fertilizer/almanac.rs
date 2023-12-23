use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberRange {
    pub start: usize,
    pub length: usize
}

trait ResourceTryMap {
    fn try_map(&self, input: usize) -> Option<usize>;
}

trait ResourceMap {
    fn map(&self, input: usize) -> usize;
    fn map_range(&self, range: NumberRange) -> Vec<NumberRange>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AlmanacMapRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl ResourceTryMap for AlmanacMapRange {
    fn try_map(&self, input: usize) -> Option<usize> {
        if input >= self.source_range_start && input < self.source_range_start + self.range_length {
            Some(self.destination_range_start + (input - self.source_range_start))
        } else {
            None
        }
    }
}

impl From<&str> for AlmanacMapRange {
    fn from(value: &str) -> Self {
        let numbers = value.split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        Self {
            destination_range_start: numbers[0],
            source_range_start: numbers[1],
            range_length: numbers[2]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AlmanacMap(Vec<AlmanacMapRange>);

impl ResourceMap for AlmanacMap {
    fn map(&self, input: usize) -> usize {
        for map_range in self.0.iter() {
            if let Some(output) = map_range.try_map(input) {
                return output;
            }
        }

        input
    }

    fn map_range(&self, range: NumberRange) -> Vec<NumberRange> {
        let mut starts: Vec<usize> = self.0.iter()
            .filter_map(|x| {
                if x.source_range_start >= range.start && x.source_range_start < range.start + range.length {
                    Some(x.source_range_start)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        starts.push(range.start);
        starts.sort();

        let starts_clone = starts.clone();

        let mut ranges = starts.iter().zip(starts_clone.iter().skip(1))
            .map(|(prev, curr)| NumberRange { start: *prev, length: *curr - *prev })
            .collect::<Vec<NumberRange>>();
        ranges.push(NumberRange { start: *starts.last().unwrap(), length: range.start + range.length - *starts.last().unwrap() });

        ranges.into_iter().map(|range| NumberRange { start: self.map(range.start), length: range.length }).collect()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct AlmanacSection {
    name: String,
    map: AlmanacMap,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    pub name: String,
    pub number: usize
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceRange {
    pub name: String,
    pub number_range: NumberRange
}

#[derive(Clone, PartialEq, Eq)]
pub struct Almanac {
    converters: HashMap<String, AlmanacSection>,
}

impl Almanac {
    pub fn map(&self, resource: &Resource) -> Resource {
        let map = self.converters.get(&resource.name).unwrap();
        let name = map.name.clone();
        let number = map.map.map(resource.number);

        Resource {
            name,
            number
        }
    }

    pub fn map_range(&self, resource_range: &ResourceRange) -> Vec<ResourceRange> {
        let map = self.converters.get(&resource_range.name).unwrap();
        let name = map.name.clone();
        map.map.map_range(resource_range.number_range).into_iter()
            .map(|number_range| ResourceRange { name: name.clone(), number_range })
            .collect()
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let mut converters: HashMap<String, AlmanacSection> = HashMap::new();

        while let Some((name, section)) = get_almanac_section(&mut lines) {
            converters.insert(name, section);
        }

        Almanac {
            converters
        }
    }
}

fn get_almanac_section(lines: &mut Lines) -> Option<(String, AlmanacSection)> {
    let mut map: Vec<AlmanacMapRange> = Vec::new();

    if let Some(line) = lines.next() {
        let (from, to) = line
            .strip_suffix(" map:").unwrap()
            .split_once("-to-").unwrap();

        while let Some(line) = lines.next() {
            if line == "" {
                return Some((from.to_string(), AlmanacSection { name: to.to_string(), map: AlmanacMap(map) }))
            }

            map.push(line.into());
        }

        Some((from.to_string(), AlmanacSection { name: to.to_string(), map: AlmanacMap(map) }))
    } else {
        None
    }
}

