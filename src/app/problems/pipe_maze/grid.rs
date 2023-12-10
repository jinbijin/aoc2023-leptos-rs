use std::ops::{Index, IndexMut};
use super::{grid_coords::GridCoords, grid_direction::GridDirection, pipe::Pipe, pipe_location::PipeLocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridHeading {
    position: GridCoords,
    heading: GridDirection
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    squares: Vec<Vec<Option<[GridDirection; 2]>>>,
    start: GridCoords,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn loop_iter(&self) -> impl Iterator<Item = PipeLocation> + '_ {
        LoopIter {
            grid: &self,
            heading: GridHeading {
                position: self.start,
                heading: self[self.start].unwrap()[0],
            },
            first: true
        }
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = GridCoords> + '_ {
        self.squares.iter().enumerate()
            .flat_map(|(y, ss)| {
                ss.iter().enumerate().map(move |(x, _)| GridCoords { x, y })
            })
    }

    #[cfg(any(ssr, test))]
    fn neighbors(&self, coords: GridCoords) -> impl Iterator<Item = GridCoords> + '_ {
        GridDirection::all().filter_map(move |direction| self.square_in_direction(coords, direction))
    }

    fn square_in_direction(&self, coords: GridCoords, direction: GridDirection) -> Option<GridCoords> {
        match direction {
            GridDirection::North => (coords.y > 0)
                .then(|| GridCoords { x: coords.x, y: coords.y - 1 }),
            GridDirection::East => (coords.x < self.width - 1)
                .then(|| GridCoords { x: coords.x + 1, y: coords.y } ),
            GridDirection::South => (coords.y < self.height - 1)
                .then(|| GridCoords { x: coords.x, y: coords.y + 1 }),
            GridDirection::West => (coords.x > 0)
                .then(|| GridCoords { x: coords.x - 1, y: coords.y })
        }
    }

    fn next_heading(&self, heading: GridHeading) -> GridHeading {
        let position = self.square_in_direction(heading.position, heading.heading).unwrap();
        let headings: Vec<_> = self[position].unwrap().into_iter()
            .filter(|direction| *direction != heading.heading.opposite())
            .collect();
        let heading = headings[0];

        GridHeading {
            position,
            heading
        }
    }
}

impl From<Vec<Vec<Pipe>>> for Grid {
    fn from(value: Vec<Vec<Pipe>>) -> Self {
        let height = value.len();
        let mut squares: Vec<Vec<Option<[GridDirection; 2]>>> = Vec::new();
        let mut start: Option<GridCoords> = None;
        let mut width = 0usize;

        for (y, ps) in value.into_iter().enumerate() {
            let mut row: Vec<Option<[GridDirection; 2]>> = Vec::new();
            if y == 0 {
                width = ps.len();
            }

            for (x, p) in ps.into_iter().enumerate() {
                match p {
                    Pipe::Some(directions) => row.push(Some(directions)),
                    Pipe::None => row.push(None),
                    Pipe::Start => {
                        row.push(None); // we fix this below
                        start = Some(GridCoords { x, y })
                    }
                }
            }
            squares.push(row);
        }

        let mut grid = Grid {
            width,
            height,
            start: start.unwrap(),
            squares
        };

        let start = grid.start;
        let directions: Vec<_> = GridDirection::all()
            .filter(|direction|
                grid.square_in_direction(start, *direction)
                    .is_some_and(|neighbor|
                        grid[neighbor].is_some_and(|nds|
                            nds.into_iter().any(|nd| grid.square_in_direction(neighbor, nd).is_some_and(|x| x == start)) ))
            ).
            collect();
        let directions = [directions[0], directions[1]];

        grid[start] = Some(directions);

        grid
    }
}

impl Index<GridCoords> for Grid {
    type Output = Option<[GridDirection; 2]>;

    fn index(&self, index: GridCoords) -> &Self::Output {
        &self.squares[index.y][index.x]
    }
}

impl IndexMut<GridCoords> for Grid {
    fn index_mut(&mut self, index: GridCoords) -> &mut Self::Output {
        &mut self.squares[index.y][index.x]
    }
}

pub struct LoopIter<'a> {
    grid: &'a Grid,
    heading: GridHeading,
    first: bool
}

impl<'a> Iterator for LoopIter<'a> {
    type Item = PipeLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first && self.heading.position == self.grid.start {
            return None;
        }
        let heading = self.heading;

        self.heading = self.grid.next_heading(self.heading);
        self.first = false;

        Some(PipeLocation {
            coords: heading.position,
            directions: self.grid[heading.position].unwrap()
        })
    }
}