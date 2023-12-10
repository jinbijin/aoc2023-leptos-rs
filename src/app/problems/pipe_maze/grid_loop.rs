use std::collections::HashMap;
use crate::app::problems::pipe_maze::grid_coords::GridCoords;
use crate::app::problems::pipe_maze::grid_direction::GridDirection;
use super::pipe_location::PipeLocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridLoop{
    pipes: Vec<PipeLocation>,
    pipe_set: HashMap<GridCoords, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopPipeResult {
    SegmentStart(GridDirection),
    SegmentContinue,
    SegmentEnd(GridDirection),
    Singleton,
    None
}

impl GridLoop {
    fn len(&self) -> usize {
        self.pipes.len()
    }

    pub fn diameter(&self) -> usize {
        self.len() / 2
    }

    pub fn get_pipe(&self, coords: GridCoords) -> LoopPipeResult {
        if let Some(index) = self.pipe_set.get(&coords) {
            let pipe = self.pipes[*index];
            if pipe.directions.into_iter().any(|direction| direction == GridDirection::East) {
                if let Some(direction) = pipe.directions.into_iter()
                    .find(|direction| *direction != GridDirection::East && *direction != GridDirection::West) {
                    LoopPipeResult::SegmentStart(direction)
                } else {
                    LoopPipeResult::SegmentContinue
                }
            } else {
                if pipe.directions.into_iter().any(|direction| direction == GridDirection::West) {
                    let direction = pipe.directions.into_iter()
                        .find(|direction| *direction != GridDirection::East && *direction != GridDirection::West)
                        .unwrap();
                    LoopPipeResult::SegmentEnd(direction)
                } else {
                    LoopPipeResult::Singleton
                }
            }
        } else {
            LoopPipeResult::None
        }
    }
}

impl FromIterator<PipeLocation> for GridLoop {
    fn from_iter<T: IntoIterator<Item=PipeLocation>>(iter: T) -> Self {
        let pipes: Vec<PipeLocation> = iter.into_iter().collect();
        let mut pipe_set: HashMap<GridCoords, usize> = HashMap::new();
        for (index, pipe) in pipes.iter().enumerate() {
            pipe_set.insert(pipe.coords, index);
        }

        Self {
            pipes,
            pipe_set
        }
    }
}
