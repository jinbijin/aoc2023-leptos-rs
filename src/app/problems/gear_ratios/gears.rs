use std::collections::HashSet;
use std::iter::Peekable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GearLocation {
    pub value: usize,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SymbolLocation {
    symbol: char,
    line: usize,
    position: usize,
}

impl SymbolLocation {
    fn is_adjacent_to(&self, gear: &GearLocation) -> bool {
        gear.line.abs_diff(self.line) <= 1
            && gear.start <= self.position + 1
            && gear.end + 1 >= self.position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GearRatio {
    symbol: SymbolLocation,
    parts: [GearLocation; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SchematicItem {
    Gear(GearLocation),
    Symbol(SymbolLocation)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schematic {
    items: HashSet<SchematicItem>
}

impl Schematic {
    pub fn get_parts_adjacent_to_symbol(&self) -> HashSet<GearLocation> {
        self.items.iter().filter_map(|item| {
            if let SchematicItem::Gear(location) = item {
                if self.items.iter().any(|symbol| {
                    if let SchematicItem::Symbol(symbol_location) = symbol {
                        symbol_location.is_adjacent_to(location)
                    } else {
                        false
                    }
                }) {
                    Some(*location)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

    pub fn get_gears(&self) -> usize {
        self.items.iter().filter_map(|item| {
            if let SchematicItem::Symbol(symbol_location) = item {
                if symbol_location.symbol == '*' {
                    let parts = self.items.iter().filter_map(|other_item| {
                        if let SchematicItem::Gear(part_location) = other_item {
                            if symbol_location.is_adjacent_to(part_location) {
                                Some(part_location)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }).collect::<Vec<&GearLocation>>();
                    if parts.len() == 2 {
                        Some(parts.into_iter().map(|x| x.value).product::<usize>())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).sum()
    }
}

pub fn read_gear_schematic(input: &str) -> Schematic {
    let items = input.lines().enumerate()
        .flat_map(|(line_number, line)| SchematicItemIterator::new(line.chars(), line_number))
        .collect::<HashSet<SchematicItem>>();
    Schematic {
        items
    }
}

struct SchematicItemIterator<T: Iterator<Item = char>> {
    iter: Peekable<T>,
    line: usize,
    pos: usize,
}

impl<T: Iterator<Item = char>> SchematicItemIterator<T> {
    fn new(iter: T, line: usize) -> Self {
        Self {
            iter: iter.peekable(),
            line,
            pos: 0
        }
    }

    fn next_internal(&mut self) -> Option<char> {
        let next = self.iter.next();
        if next.is_some() {
            self.pos += 1;
        }
        next
    }
}

impl<T: Iterator<Item = char>> Iterator for SchematicItemIterator<T> {
    type Item = SchematicItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.line;
            let current_pos = self.pos;
            if let Some(next_char) = self.next_internal() {
                if next_char.is_digit(10) {
                    let mut char_vec = vec![next_char];
                    while self.iter.peek().is_some_and(|c| c.is_digit(10))  {
                        let next_char = self.next_internal().unwrap();
                        char_vec.push(next_char);
                    }
                    let length = char_vec.len();
                    let value = char_vec.into_iter().collect::<String>().parse::<usize>().unwrap();
                    return Some(SchematicItem::Gear(GearLocation { value, line, start: current_pos, end: current_pos + length - 1 }));
                } else if next_char == '.' {
                    // Continue loop
                } else {
                    return Some(SchematicItem::Symbol(SymbolLocation { symbol: next_char, line, position: current_pos }));
                }
            } else {
                return None;
            }
        }
    }
}
