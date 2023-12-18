// Note:
// Conversion: 1 meter = 2 units
// The starting point is the grid point (0, 0), with the grid square between (-1, -1) and (1, 1) already cut out.

use std::collections::BTreeSet;
use super::dig_plan::{DigDirection, DigPlanStep};

/// A measuring line
///
/// It describes a horizontal line with fixed y.
/// It is used only for odd y; such lines will always intersect all segments transversally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MeasuringLine {
    y: isize,
}

impl MeasuringLine {
    fn intersection(&self, segment: &TrenchEdgeSegment) -> Option<TerrainCoords> {
        match segment {
            TrenchEdgeSegment::Vertical { x, y_from, y_to } =>
                if *y_from <= self.y && self.y <= *y_to {
                    Some(TerrainCoords::new(*x, self.y))
                } else {
                    None
                },
            TrenchEdgeSegment::Horizontal { x_from: _, x_to: _, y: _ } => None,
        }
    }

    /// The length of the horizontal projection of the measuring line in the interior of the trench
    fn len_inside(&self, edge: &TrenchEdge) -> isize {
        let mut intersections_x: Vec<_> = edge.0.iter()
            .filter_map(move |s| self.intersection(s))
            .map(|i| i.x)
            .collect();
        intersections_x.sort();

        let mut length = 0isize;
        while intersections_x.len() > 0 {
            let top_two = intersections_x.split_off(intersections_x.len() - 2);
            length += top_two[1] - top_two[0];
        }

        length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct TerrainCoords {
    x: isize,
    y: isize
}

impl TerrainCoords {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TrenchEdgeSegment {
    Vertical { x: isize, y_from: isize, y_to: isize },
    Horizontal { x_from: isize, x_to: isize, y: isize },
}

impl TrenchEdgeSegment {
    fn len(&self) -> isize {
        match self {
            TrenchEdgeSegment::Vertical { x: _, y_from, y_to } => *y_to - *y_from,
            TrenchEdgeSegment::Horizontal { x_from, x_to, y: _ } => *x_to - *x_from,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrenchEdge(Vec<TrenchEdgeSegment>);

impl TrenchEdge {
    pub fn from(dig_plan_steps: &Vec<DigPlanStep>) -> Self {
        let mut builder = TrenchEdgeBuilder::from(dig_plan_steps);

        while let Some(()) = builder.step() {}

        let left_hand_edge = Self(builder.left_hand_edge);
        let right_hand_edge = Self(builder.right_hand_edge);

        // The longer cycle is the outer one
        if left_hand_edge.len() > right_hand_edge.len() {
            left_hand_edge
        } else {
            right_hand_edge
        }
    }

    fn len(&self) -> isize {
        self.0.iter().map(|x| x.len()).sum()
    }

    pub fn area(&self) -> isize {
        let ys: BTreeSet<_> = self.0.iter()
            .filter_map(|s| match s {
                TrenchEdgeSegment::Horizontal { x_from: _, x_to: _, y } => Some(*y),
                TrenchEdgeSegment::Vertical { x: _, y_from: _, y_to: _ } => None,
            })
            .collect();

        ys.iter().zip(ys.iter().skip(1))
            .map(|(y_from, y_to)| (*y_from, *y_to))
            .map(move |(y_from, y_to)| {
                let measuring_line = MeasuringLine { y: y_from + 1 };
                (y_to - y_from) * measuring_line.len_inside(self) / 4
            })
            .sum()
    }
}

struct TrenchEdgeBuilder<'a> {
    dig_plan_steps: &'a Vec<DigPlanStep>,

    current: TerrainCoords,
    step_index: usize,
    step_window: [DigPlanStep; 3],
    left_hand_edge: Vec<TrenchEdgeSegment>,
    right_hand_edge: Vec<TrenchEdgeSegment>,
}

impl<'a> TrenchEdgeBuilder<'a> {
    fn from(dig_plan_steps: &'a Vec<DigPlanStep>) -> Self {
        TrenchEdgeBuilder {
            dig_plan_steps,

            current: TerrainCoords::default(),
            step_index: 0,
            step_window: [*dig_plan_steps.last().unwrap(), dig_plan_steps[0], dig_plan_steps[1]],
            left_hand_edge: Vec::new(),
            right_hand_edge: Vec::new()
        }
    }

    fn step(&mut self) -> Option<()> {
        if self.step_index == self.dig_plan_steps.len() {
            return None;
        }

        match self.step_window[1].direction {
            DigDirection::Right => {
                let x_offset = 2 * self.step_window[1].length;
                let lhs_offset_start = if self.step_window[0].direction == DigDirection::Down { 1isize } else { -1isize };
                let lhs_offset_end = if self.step_window[2].direction == DigDirection::Down { 1isize } else { -1isize };

                self.left_hand_edge.push(TrenchEdgeSegment::Horizontal {
                    x_from: self.current.x + lhs_offset_start,
                    x_to: self.current.x + x_offset + lhs_offset_end,
                    y: self.current.y - 1,
                });
                self.right_hand_edge.push(TrenchEdgeSegment::Horizontal {
                    x_from: self.current.x - lhs_offset_start,
                    x_to: self.current.x + x_offset - lhs_offset_end,
                    y: self.current.y + 1,
                });
                self.current.x += x_offset;
            },
            DigDirection::Left => {
                let x_offset = -2 * self.step_window[1].length;
                let lhs_offset_start = if self.step_window[0].direction == DigDirection::Down { 1isize } else { -1isize };
                let lhs_offset_end = if self.step_window[2].direction == DigDirection::Down { 1isize } else { -1isize };

                self.left_hand_edge.push(TrenchEdgeSegment::Horizontal {
                    x_from: self.current.x + x_offset + lhs_offset_end,
                    x_to: self.current.x + lhs_offset_start,
                    y: self.current.y + 1,
                });
                self.right_hand_edge.push(TrenchEdgeSegment::Horizontal {
                    x_from: self.current.x + x_offset - lhs_offset_end,
                    x_to: self.current.x - lhs_offset_start,
                    y: self.current.y - 1,
                });
                self.current.x += x_offset;
            },
            DigDirection::Down => {
                let y_offset = 2 * self.step_window[1].length;
                let lhs_offset_start = if self.step_window[0].direction == DigDirection::Right { -1isize } else { 1isize };
                let lhs_offset_end = if self.step_window[2].direction == DigDirection::Right { -1isize } else { 1isize };

                self.left_hand_edge.push(TrenchEdgeSegment::Vertical {
                    x: self.current.x + 1,
                    y_from: self.current.y + lhs_offset_start,
                    y_to: self.current.y + y_offset + lhs_offset_end
                });
                self.right_hand_edge.push(TrenchEdgeSegment::Vertical {
                    x: self.current.x - 1,
                    y_from: self.current.y - lhs_offset_start,
                    y_to: self.current.y + y_offset - lhs_offset_end
                });
                self.current.y += y_offset;
            },
            DigDirection::Up => {
                let y_offset = -2 * self.step_window[1].length;
                let lhs_offset_start = if self.step_window[0].direction == DigDirection::Right { -1isize } else { 1isize };
                let lhs_offset_end = if self.step_window[2].direction == DigDirection::Right { -1isize } else { 1isize };

                self.left_hand_edge.push(TrenchEdgeSegment::Vertical {
                    x: self.current.x - 1,
                    y_from: self.current.y + y_offset + lhs_offset_end,
                    y_to: self.current.y + lhs_offset_start
                });
                self.right_hand_edge.push(TrenchEdgeSegment::Vertical {
                    x: self.current.x + 1,
                    y_from: self.current.y + y_offset - lhs_offset_end,
                    y_to: self.current.y - lhs_offset_start
                });
                self.current.y += y_offset;
            },
        }

        self.increment_step_state();
        Some(())
    }

    fn increment_step_state(&mut self) {
        let next_step = self.dig_plan_steps[(self.step_index + 2) % self.dig_plan_steps.len()];

        self.step_index += 1;
        self.step_window = [self.step_window[1], self.step_window[2], next_step];
    }
}