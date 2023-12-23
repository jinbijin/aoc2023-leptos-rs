use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
pub struct Scratchcard {
    winning_numbers: HashSet<usize>,
    own_numbers: Vec<usize>,
}

impl Scratchcard {
    fn new(winning_numbers: HashSet<usize>, own_numbers: Vec<usize>) -> Scratchcard {
        Scratchcard {
            winning_numbers,
            own_numbers,
        }
    }

    pub fn value(&self) -> usize {
        let numbers = self.own_numbers.iter()
            .filter(|own_number| self.winning_numbers.contains(own_number))
            .map(|_| 2)
            .collect::<Vec<usize>>();
        if numbers.is_empty() {
            0
        } else {
            numbers.iter().skip(1).product()
        }
    }

    fn matching_numbers(&self) -> usize {
        self.own_numbers.iter()
            .filter(|own_number| self.winning_numbers.contains(own_number))
            .count()
    }
}

impl From<&str> for Scratchcard {
    fn from(value: &str) -> Self {
        let (_, card_def) = value.split_once(": ").unwrap();
        let (winning_numbers_def, own_numbers_def) = card_def.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_def.chars().card_values().collect::<HashSet<usize>>();
        let own_numbers = own_numbers_def.chars().card_values().collect::<Vec<usize>>();

        Scratchcard::new(winning_numbers, own_numbers)
    }
}

struct CardValueIterator<T: Iterator<Item = char>> {
    iter: T
}

impl<T: Iterator<Item = char>> Iterator for CardValueIterator<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first) = self.iter.next() {
            let mut combined = String::with_capacity(2);
            if first != ' ' {
                combined.push(first);
            }

            let second = self.iter.next().unwrap();
            combined.push(second);

            self.iter.next();

            Some(combined.parse::<usize>().unwrap())
        } else {
            None
        }
    }
}

trait AsCardValues<T: Iterator<Item = char>> {
    fn card_values(self) -> CardValueIterator<T>;
}

impl<T: Iterator<Item = char>> AsCardValues<T> for T {
    fn card_values(self) -> CardValueIterator<T> {
        CardValueIterator { iter: self }
    }
}

pub struct ScratchcardCopyIterator<T: Iterator<Item = Scratchcard>> {
    iter: T,
    extra_copies: VecDeque<usize>,
}

impl<T: Iterator<Item = Scratchcard>> Iterator for ScratchcardCopyIterator<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let card_count = 1 + self.extra_copies.pop_front().unwrap_or_default();

        if let Some(card) = self.iter.next() {
            let value = card.matching_numbers();
            for index in 0..value {
                if let Some(copy_count) = self.extra_copies.get_mut(index) {
                    *copy_count += card_count;
                } else {
                    self.extra_copies.push_back(card_count);
                }
            }

            Some(card_count)
        } else {
            None
        }
    }
}

pub trait AsScratchcardCopy<T: Iterator<Item = Scratchcard>> {
    fn process_copies(self) -> ScratchcardCopyIterator<T>;
}

impl<T: Iterator<Item = Scratchcard>> AsScratchcardCopy<T> for T {
    fn process_copies(self) -> ScratchcardCopyIterator<T> {
        ScratchcardCopyIterator {
            iter: self,
            extra_copies: VecDeque::new()
        }
    }
}
