use super::{grid_coords::GridCoords, grid_direction::GridDirection, grid_loop::{GridLoop, LoopPipeResult}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridRegionType {
    Outside,
    Inside,
    OnLoop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridRegionResult {
    pub region_type: GridRegionType,
    pub coords: GridCoords,
}

#[derive(Debug)]
pub struct GridRegionIter<'a, T: Iterator<Item = GridCoords> + Sized> {
    coords_iter: T,
    grid_loop: &'a GridLoop,
    inside_loop: bool,
    loop_start: Option<GridDirection>
}

impl<'a, T: Iterator<Item = GridCoords> + Sized> Iterator for GridRegionIter<'a, T> {
    type Item = GridRegionResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(coords) = self.coords_iter.next() {
            let region_type = match self.grid_loop.get_pipe(coords) {
                LoopPipeResult::Singleton => {
                    self.inside_loop = !self.inside_loop;
                    GridRegionType::OnLoop
                }
                LoopPipeResult::SegmentStart(direction) => {
                    self.loop_start = Some(direction);
                    GridRegionType::OnLoop
                },
                LoopPipeResult::SegmentContinue => {
                    GridRegionType::OnLoop
                },
                LoopPipeResult::SegmentEnd(direction) => {
                    if self.loop_start == Some(direction.opposite()) {
                        self.inside_loop = !self.inside_loop;
                    }
                    self.loop_start = None;
                    GridRegionType::OnLoop
                },
                LoopPipeResult::None => {
                    if self.inside_loop { GridRegionType::Inside } else { GridRegionType::Outside }
                }
            };

            Some(GridRegionResult {
                coords,
                region_type,
            })
        } else {
            None
        }
    }
}

pub trait AsGridRegionIter: Iterator<Item = GridCoords> + Sized {
    fn grid_region_iter(self, grid_loop: &GridLoop) -> GridRegionIter<Self>;
}

impl<T: Iterator<Item = GridCoords> + Sized> AsGridRegionIter for T {
    fn grid_region_iter(self, grid_loop: &GridLoop) -> GridRegionIter<Self> {
        GridRegionIter {
            coords_iter: self,
            grid_loop,
            inside_loop: false,
            loop_start: None
        }
    }
}
