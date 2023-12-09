use std::collections::HashSet;
use crate::app::math::{Gcd, Mod};

#[derive(Debug, Clone, PartialEq)]
pub struct EventuallyPeriodic {
    pub initial_part: Vec<usize>,
    pub initial_length: usize,
    pub repeating_part: Vec<usize>,
    pub repeating_length: usize,
}

impl EventuallyPeriodic {
    pub fn get_first_index(&self) -> Option<usize> {
        if let Some(index) = self.initial_part.first() {
            Some(*index)
        } else if let Some(index) = self.repeating_part.first() {
            Some(*index + self.initial_length)
        } else {
            None
        }
    }

    fn shift_repeating(&mut self, by: usize) {
        let copies = by / self.repeating_length;
        for i in 0..copies {
            let mut to_append: Vec<_> = self.repeating_part.iter()
                .map(|x| *x + self.initial_length + i * self.repeating_length)
                .collect();
            self.initial_part.append(&mut to_append);
        }

        let mut to_append: Vec<_> = self.repeating_part.iter()
            .filter_map(|x| if *x < (by % self.repeating_length) { Some(*x + self.initial_length + copies * self.repeating_length)} else { None })
            .collect();
        self.initial_part.append(&mut to_append);

        self.initial_length += by;

        let mut repeating_part: Vec<_> = self.repeating_part.iter()
            .filter_map(|x| if *x >= (by % self.repeating_length) { Some(*x - (by % self.repeating_length)) } else { None })
            .collect();
        let mut shifted_repeated: Vec<_> =  self.repeating_part.iter()
            .filter_map(|x| if *x < (by % self.repeating_length) { Some(*x - (by % self.repeating_length) + self.repeating_length) } else { None })
            .collect();
        repeating_part.append(&mut shifted_repeated);

        self.repeating_part = repeating_part;
    }

    pub fn mul(mut self, mut rhs: Self) -> Self {
        if self.initial_length < rhs.initial_length {
            self.shift_repeating(rhs.initial_length - self.initial_length);
        } else if self.initial_length > rhs.initial_length {
            rhs.shift_repeating(self.initial_length - rhs.initial_length);
        }
        let initial_length = self.initial_length; // now that we made those equal

        let initial_lhs: HashSet<_> = HashSet::from_iter(self.initial_part.iter().map(|x| *x));
        let initial_rhs: HashSet<_> = HashSet::from_iter(rhs.initial_part.iter().map(|x| *x));
        let mut initial_part: Vec<_> = initial_lhs.intersection(&initial_rhs).map(|x| *x).collect();
        initial_part.sort();

        let mut repeating_part: Vec<_> = self.repeating_part.iter()
            .map(|x| Mod { value: *x, modulo: self.repeating_length })
            .flat_map(|lhs| {
                rhs.repeating_part.iter()
                    .map(|x| Mod { value: *x, modulo: rhs.repeating_length })
                    .filter_map(move |rhs| lhs.try_mul(rhs))
            })
            .map(|x| x.value)
            .collect();
        repeating_part.sort();

        let gcd = self.repeating_length.gcd(rhs.repeating_length).gcd;
        let repeating_length = (self.repeating_length / gcd) * rhs.repeating_length;

        EventuallyPeriodic {
            initial_length,
            initial_part,
            repeating_length,
            repeating_part
        }
    }
}
