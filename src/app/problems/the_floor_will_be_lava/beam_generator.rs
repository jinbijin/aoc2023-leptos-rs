use std::collections::HashSet;
use super::{beams::BeamSegment, contraption::{BarrierType, Contraption}};

#[derive(Debug, Clone)]
pub struct BeamCollection<'a> {
    contraption: &'a Contraption,
    beam_segments: HashSet<BeamSegment>
}

impl<'a> BeamCollection<'a> {
    pub fn new(contraption: &'a Contraption, beam_segments: HashSet<BeamSegment>) -> BeamCollection<'a> {
        Self {
            contraption,
            beam_segments
        }
    }

    pub fn energized_tile_count(&self) -> usize {
        let mut nodes: HashSet<(usize, usize)> = HashSet::with_capacity(self.beam_segments.len());
        let mut processed_segments: Vec<BeamSegment> = Vec::with_capacity(self.beam_segments.len());
        let mut internal_length = 0usize;
        let mut intersection_count = 0usize;

        for segment in self.beam_segments.iter() {
            match segment {
                BeamSegment::Horizontal { y, x_from, x_to } => {
                    if let Some(x_from) = x_from {
                        nodes.insert((*x_from, *y));
                    }
                    if let Some(x_to) = x_to {
                        nodes.insert((*x_to, *y));
                    }
                },
                BeamSegment::Vertical { x, y_from, y_to } => {
                    if let Some(y_from) = y_from {
                        nodes.insert((*x, *y_from));
                    }
                    if let Some(y_to) = y_to {
                        nodes.insert((*x, *y_to));
                    }
                }
            }

            internal_length += self.internal_length(*segment);
            intersection_count += processed_segments.iter().filter(|s| s.intersects(segment)).count();

            processed_segments.push(*segment);
        }

        internal_length + nodes.len() - intersection_count
    }

    fn internal_length(&self, beam_segment: BeamSegment) -> usize {
        // Ordinalize to avoid underflows
        match beam_segment {
            BeamSegment::Horizontal { y: _, x_from, x_to } => {
                let x_from = x_from.map_or(0, |x| x + 1);
                let x_to = x_to.map_or(self.contraption.width + 1, |x| x + 1);

                x_to - x_from - 1
            },
            BeamSegment::Vertical { x: _, y_from, y_to } => {
                let y_from = y_from.map_or(0, |y| y + 1);
                let y_to = y_to.map_or(self.contraption.height + 1, |y| y + 1);

                y_to - y_from - 1
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamGeneratorState {
    HorizontalForward { y: usize, x_from: Option<usize> },
    HorizontalBackward { y: usize, x_to: Option<usize> },
    VerticalForward { x: usize, y_from: Option<usize> },
    VerticalBackward { x: usize, y_to: Option<usize> },
}

pub struct BeamGenerator<'a> {
    contraption: &'a Contraption,
    states: Vec<BeamGeneratorState>,
    beam_segments: HashSet<BeamSegment>
}

impl<'a> BeamGenerator<'a> {
    pub fn from(contraption: &'a Contraption, initial_state: BeamGeneratorState) -> Self {
        Self {
            contraption,
            states: vec![initial_state],
            beam_segments: HashSet::new()
        }
    }

    pub fn process(mut self) -> BeamCollection<'a> {
        while self.process_top_state().is_some() {}

        BeamCollection::new(self.contraption, self.beam_segments)
    }

    fn process_top_state(&mut self) -> Option<()> {
        if let Some(state) = self.states.pop() {
            match state {
                BeamGeneratorState::HorizontalForward { y, x_from } => {
                    self.process_horizontal_forward(y, x_from);
                },
                BeamGeneratorState::HorizontalBackward { y, x_to } => {
                    self.process_horizontal_backward(y, x_to);
                },
                BeamGeneratorState::VerticalForward { x, y_from } => {
                    self.process_vertical_forward(x, y_from);
                },
                BeamGeneratorState::VerticalBackward { x, y_to } => {
                    self.process_vertical_backward(x, y_to);
                },
            }

            Some(())
        } else {
            None
        }
    }

    fn process_horizontal_forward(&mut self, y: usize, x_from: Option<usize>) {
        if let Some(node) = self.contraption.next_horizontal(y, x_from) {
            let beam_segment = BeamSegment::Horizontal {
                y,
                x_from,
                x_to: Some(node.x)
            };
            if !self.beam_segments.insert(beam_segment) {
                return;
            }

            match node.barrier_type {
                BarrierType::MirrorForward => {
                    self.states.push(BeamGeneratorState::VerticalBackward { x: node.x, y_to: Some(node.y) });
                },
                BarrierType::MirrorBackward => {
                    self.states.push(BeamGeneratorState::VerticalForward { x: node.x, y_from: Some(node.y) });
                },
                BarrierType::SplitterHorizontal => {
                    self.states.push(BeamGeneratorState::HorizontalForward { y: node.y, x_from: Some(node.x) });
                },
                BarrierType::SplitterVertical => {
                    self.states.push(BeamGeneratorState::VerticalBackward { x: node.x, y_to: Some(node.y) });
                    self.states.push(BeamGeneratorState::VerticalForward { x: node.x, y_from: Some(node.y) });
                }
            }
        } else {
            let beam_segment = BeamSegment::Horizontal {
                y,
                x_from,
                x_to: None
            };
            self.beam_segments.insert(beam_segment);
        }
    }

    fn process_horizontal_backward(&mut self, y: usize, x_to: Option<usize>) {
        if let Some(node) = self.contraption.previous_horizontal(y, x_to) {
            let beam_segment = BeamSegment::Horizontal {
                y,
                x_to,
                x_from: Some(node.x),
            };
            if !self.beam_segments.insert(beam_segment) {
                return;
            }

            match node.barrier_type {
                BarrierType::MirrorForward => {
                    self.states.push(BeamGeneratorState::VerticalForward { x: node.x, y_from: Some(node.y) });
                },
                BarrierType::MirrorBackward => {
                    self.states.push(BeamGeneratorState::VerticalBackward { x: node.x, y_to: Some(node.y) });
                },
                BarrierType::SplitterHorizontal => {
                    self.states.push(BeamGeneratorState::HorizontalBackward { y: node.y, x_to: Some(node.x) });
                },
                BarrierType::SplitterVertical => {
                    self.states.push(BeamGeneratorState::VerticalBackward { x: node.x, y_to: Some(node.y) });
                    self.states.push(BeamGeneratorState::VerticalForward { x: node.x, y_from: Some(node.y) });
                }
            }
        } else {
            let beam_segment = BeamSegment::Horizontal {
                y,
                x_to,
                x_from: None
            };
            self.beam_segments.insert(beam_segment);
        }
    }

    fn process_vertical_forward(&mut self, x: usize, y_from: Option<usize>) {
        if let Some(node) = self.contraption.next_vertical(x, y_from) {
            let beam_segment = BeamSegment::Vertical {
                x,
                y_from,
                y_to: Some(node.y)
            };
            if !self.beam_segments.insert(beam_segment) {
                return;
            }

            match node.barrier_type {
                BarrierType::MirrorForward => {
                    self.states.push(BeamGeneratorState::HorizontalBackward { y: node.y, x_to: Some(node.x) });
                },
                BarrierType::MirrorBackward => {
                    self.states.push(BeamGeneratorState::HorizontalForward { y: node.y, x_from: Some(node.x) });
                },
                BarrierType::SplitterHorizontal => {
                    self.states.push(BeamGeneratorState::HorizontalBackward { y: node.y, x_to: Some(node.x) });
                    self.states.push(BeamGeneratorState::HorizontalForward { y: node.y, x_from: Some(node.x) });
                },
                BarrierType::SplitterVertical => {
                    self.states.push(BeamGeneratorState::VerticalForward { x: node.x, y_from: Some(node.y) });
                }
            }
        } else {
            let beam_segment = BeamSegment::Vertical {
                x,
                y_from,
                y_to: None
            };
            self.beam_segments.insert(beam_segment);
        }
    }

    fn process_vertical_backward(&mut self, x: usize, y_to: Option<usize>) {
        if let Some(node) = self.contraption.previous_vertical(x, y_to) {
            let beam_segment = BeamSegment::Vertical {
                x,
                y_to,
                y_from: Some(node.y)
            };
            if !self.beam_segments.insert(beam_segment) {
                return;
            }

            match node.barrier_type {
                BarrierType::MirrorForward => {
                    self.states.push(BeamGeneratorState::HorizontalForward { y: node.y, x_from: Some(node.x) });
                },
                BarrierType::MirrorBackward => {
                    self.states.push(BeamGeneratorState::HorizontalBackward { y: node.y, x_to: Some(node.x) });
                },
                BarrierType::SplitterHorizontal => {
                    self.states.push(BeamGeneratorState::HorizontalBackward { y: node.y, x_to: Some(node.x) });
                    self.states.push(BeamGeneratorState::HorizontalForward { y: node.y, x_from: Some(node.x) });
                },
                BarrierType::SplitterVertical => {
                    self.states.push(BeamGeneratorState::VerticalBackward { x: node.x, y_to: Some(node.y) });
                }
            }
        } else {
            let beam_segment = BeamSegment::Vertical {
                x,
                y_to,
                y_from: None
            };
            self.beam_segments.insert(beam_segment);
        }
    }
}
