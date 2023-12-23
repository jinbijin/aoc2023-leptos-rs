use std::cmp::{max, Ordering};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxSet {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl BoxSet {
    pub fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

impl PartialOrd for BoxSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.red == other.red && self.green == other.green && self.blue == other.blue {
            Some(Ordering::Equal)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Add for BoxSet {
    type Output = BoxSet;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            green: max(self.green, rhs.green),
            red: max(self.red, rhs.red),
            blue: max(self.blue, rhs.blue)
        }
    }
}

impl Sum for BoxSet {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut into_iter = iter.into_iter();
        let mut box_set = BoxSet { green: 0, red: 0, blue: 0 };
        while let Some(next) = into_iter.next() {
            box_set = box_set + next;
        }

        box_set
    }
}

impl From<&str> for BoxSet {
    fn from(value: &str) -> Self {
        let items = value.split(", ");
        let mut box_set = BoxSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for item in items {
            if item.ends_with("green") {
                box_set.green = item.strip_suffix(" green").unwrap().parse::<usize>().unwrap();
            }
            if item.ends_with("red") {
                box_set.red = item.strip_suffix(" red").unwrap().parse::<usize>().unwrap();
            }
            if item.ends_with("blue") {
                box_set.blue = item.strip_suffix(" blue").unwrap().parse::<usize>().unwrap();
            }
        }
        box_set
    }
}
