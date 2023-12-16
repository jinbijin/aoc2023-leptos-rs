use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BarrierType {
    MirrorForward, // "/"
    MirrorBackward, // "\"
    SplitterHorizontal, // "-"
    SplitterVertical, // "|"
}

impl BarrierType {
    fn from(value: char) -> Option<Self> {
        match value {
            '/' => Some(BarrierType::MirrorForward),
            '\\' => Some(BarrierType::MirrorBackward),
            '-' => Some(BarrierType::SplitterHorizontal),
            '|' => Some(BarrierType::SplitterVertical),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BarrierNode {
    pub barrier_type: BarrierType,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Contraption {
    pub width: usize,
    pub height: usize,
    rows: Vec<BTreeMap<usize, BarrierNode>>,
    columns: Vec<BTreeMap<usize, BarrierNode>>,
}

impl Contraption {
    pub fn next_horizontal(&self, y: usize, x_from: Option<usize>) -> Option<BarrierNode> {
        let lower_bound = match x_from {
            Some(x_from) => Excluded(x_from),
            None => Unbounded
        };

        self.rows[y].range((lower_bound, Unbounded)).next().map(|(_, v)| *v)
    }

    pub fn previous_horizontal(&self, y: usize, x_to: Option<usize>) -> Option<BarrierNode> {
        let upper_bound = match x_to {
            Some(x_to) => Excluded(x_to),
            None => Unbounded
        };

        self.rows[y].range((Unbounded, upper_bound)).rev().next().map(|(_, v)| *v)
    }

    pub fn next_vertical(&self, x: usize, y_from: Option<usize>) -> Option<BarrierNode> {
        let lower_bound = match y_from {
            Some(y_from) => Excluded(y_from),
            None => Unbounded
        };

        self.columns[x].range((lower_bound, Unbounded)).next().map(|(_, v)| *v)
    }

    pub fn previous_vertical(&self, x: usize, y_to: Option<usize>) -> Option<BarrierNode> {
        let upper_bound = match y_to {
            Some(y_to) => Excluded(y_to),
            None => Unbounded
        };

        self.columns[x].range((Unbounded, upper_bound)).rev().next().map(|(_, v)| *v)
    }
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let mut width = 0usize;
        let mut height = 0usize;
        let mut rows: Vec<BTreeMap<usize, BarrierNode>> = Vec::new();
        let mut columns: Vec<BTreeMap<usize, BarrierNode>> = Vec::new();

        for (y, line) in value.lines().enumerate() {
            height += 1;
            rows.push(BTreeMap::new());

            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    width += 1;
                    columns.push(BTreeMap::new());
                }

                if let Some(barrier_type) = BarrierType::from(c) {
                    let barrier_node = BarrierNode {
                        barrier_type,
                        x,
                        y,
                    };
                    columns[x].insert(y, barrier_node);
                    rows[y].insert(x, barrier_node);
                }
            }
        }

        Self {
            width,
            height,
            rows,
            columns
        }
    }
}
