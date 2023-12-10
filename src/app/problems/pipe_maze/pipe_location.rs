use super::{grid_coords::GridCoords, grid_direction::GridDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipeLocation {
    pub coords: GridCoords,
    pub directions: [GridDirection; 2]
}
