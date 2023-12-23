use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::app::common::DistanceHashGraph;
use crate::create_formatted_flat_enum;

create_formatted_flat_enum! {
    ForestTileType error ParseForestTileTypeError with
    [
        Forest => "#",
        Path => ".",
        SlopeNorth => "^",
        SlopeEast => ">",
        SlopeSouth => "v",
        SlopeWest => "<"
    ]
}

impl ForestTileType {
    fn is_slope(&self) -> bool {
        *self != ForestTileType::Forest && *self != ForestTileType::Path
    }

    fn is_passable_to_north(&self, can_climb: bool) -> bool {
        *self == ForestTileType::Path || if can_climb { self.is_slope() } else {*self == ForestTileType::SlopeNorth }
    }

    fn is_passable_to_east(&self, can_climb: bool) -> bool {
        *self == ForestTileType::Path || if can_climb { self.is_slope() } else {*self == ForestTileType::SlopeEast }
    }

    fn is_passable_to_south(&self, can_climb: bool) -> bool {
        *self == ForestTileType::Path || if can_climb { self.is_slope() } else { *self == ForestTileType::SlopeSouth }
    }

    fn is_passable_to_west(&self, can_climb: bool) -> bool {
        *self == ForestTileType::Path || if can_climb { self.is_slope() } else { *self == ForestTileType::SlopeWest }
    }
}

pub struct ForestTrails {
    width: usize,
    height: usize,
    tiles: Vec<Vec<ForestTileType>>,
}

impl ForestTrails {
    pub fn starting_vertex(&self) -> (usize, usize) {
        let (starting_x, _) = self.tiles[0].iter().enumerate()
            .find(|(_, tile)| **tile == ForestTileType::Path)
            .unwrap();
        (starting_x, 0usize)
    }

    pub fn ending_vertex(&self) -> (usize, usize) {
        let (ending_x, _) = self.tiles[self.height - 1].iter().enumerate()
            .find(|(_, tile)| **tile == ForestTileType::Path)
            .unwrap();
        (ending_x, self.height - 1)
    }

    pub fn as_graph(&self, can_climb: bool) -> DistanceHashGraph<(usize, usize)> {
        let mut graph = DistanceHashGraph::new();

        let starting_vertex = self.starting_vertex();
        graph.insert_vertex(starting_vertex);

        let mut current_vertex_stack = vec![starting_vertex];
        let mut encountered_vertices = HashSet::from([starting_vertex]);
        while let Some(current_vertex) = current_vertex_stack.pop() {
            if !self.is_ending_vertex(current_vertex) {
                for branch in self.branches_at(current_vertex, can_climb) {
                    let (distance, destination) = self.next_vertex(current_vertex, branch);
                    graph.insert_edge(&current_vertex, destination, distance);
                    graph.insert_vertex(destination);

                    if !encountered_vertices.contains(&destination) {
                        current_vertex_stack.push(destination);
                        encountered_vertices.insert(destination);
                    }
                }
            }
        }

        graph
    }

    fn next_vertex(&self, start: (usize, usize), into: (usize, usize)) -> (usize, (usize, usize)) {
        let mut path = [start, into];
        let mut distance = 1usize;

        while path[0] == into || (!self.tile_type(path[0]).is_slope() && !self.is_ending_vertex(path[1]) && !self.is_starting_vertex(path[1])) {
            let next = self.branches_at(path[1], true).into_iter().find(|v| *v != path[0]).unwrap();

            path = [path[1], next];
            distance += 1;
        }

        (distance, path[1])
    }

    fn branches_at(&self, vertex: (usize, usize), can_climb: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let (x, y) = vertex;
        if x > 0 && self.tile_type((x - 1, y)).is_passable_to_west(can_climb) {
            result.push((x - 1, y));
        }
        if y > 0 && self.tile_type((x, y - 1)).is_passable_to_north(can_climb) {
            result.push((x, y - 1));
        }
        if x < self.width - 1 && self.tile_type((x + 1, y)).is_passable_to_east(can_climb) {
            result.push((x + 1, y));
        }
        if y < self.height - 1 && self.tile_type((x, y + 1)).is_passable_to_south(can_climb) {
            result.push((x, y + 1));
        }

        result
    }

    fn tile_type(&self, vertex: (usize, usize)) -> ForestTileType {
        let (x, y) = vertex;
        self.tiles[y][x]
    }

    fn is_starting_vertex(&self, vertex: (usize, usize)) -> bool {
        let (_, y) = vertex;
        y == 0
    }

    fn is_ending_vertex(&self, vertex: (usize, usize)) -> bool {
        let (_, y) = vertex;
        y == self.height - 1
    }
}

impl FromStr for ForestTrails {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let mut height = 0usize;
        let mut tiles: Vec<Vec<ForestTileType>> = Vec::new();

        for (y, line) in s.lines().enumerate() {
            height += 1;

            let mut row: Vec<ForestTileType> = Vec::new();

            for c in line.split("").filter(|x| !x.is_empty()) {
                if y == 0 {
                    width += 1;
                }

                row.push(c.parse::<ForestTileType>().unwrap());
            }

            tiles.push(row);
        }

        Ok(Self {
            width,
            height,
            tiles
        })
    }
}

impl Debug for ForestTrails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForestTrails ({}x{})", self.width, self.height)?;

        for row in self.tiles.iter() {
            write!(f, "\n")?;
            for tile in row.iter() {
                write!(f, "{:?}", tile)?;
            }
        }

        Ok(())
    }
}