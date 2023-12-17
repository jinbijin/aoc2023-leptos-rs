use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GearCityDirection {
    East,
    South,
    West,
    North
}

impl GearCityDirection {
    fn all() -> impl Iterator<Item = GearCityDirection> {
        [GearCityDirection::East, GearCityDirection::South, GearCityDirection::West, GearCityDirection::North].into_iter()
    }

    fn opposite(&self) -> GearCityDirection {
        match self {
            GearCityDirection::East => GearCityDirection::West,
            GearCityDirection::South => GearCityDirection::North,
            GearCityDirection::West => GearCityDirection::East,
            GearCityDirection::North => GearCityDirection::South,
        }
    }

    fn is_opposite(&self, other: GearCityDirection) -> bool {
        other == self.opposite()
    }

    pub fn shift(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            GearCityDirection::East => (x + 1, y),
            GearCityDirection::South => (x, y + 1),
            GearCityDirection::West => (x - 1, y),
            GearCityDirection::North => (x, y - 1)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GearCityHeading {
    direction: GearCityDirection,
    time: usize
}

impl GearCityHeading {
    pub fn from(direction: GearCityDirection) -> GearCityHeading {
        GearCityHeading {
            direction,
            time: 1
        }
    }

    pub fn direction(&self) -> GearCityDirection {
        self.direction
    }

    pub fn time_in_direction(&self) -> usize {
        self.time
    }

    pub fn with(self, direction: GearCityDirection) -> GearCityHeading {
        if self.direction == direction {
            GearCityHeading {
                direction,
                time: self.time + 1
            }
        } else {
            GearCityHeading::from(direction)
        }
    }
}

#[derive(Debug)]
pub struct GearCityGrid {
    width: usize,
    height: usize,
    heat_loss_map: Vec<Vec<usize>>,
}

impl GearCityGrid {
    pub fn is_endpoint(&self, x: usize, y: usize) -> bool {
        x == self.width - 1 && y == self.height - 1
    }

    pub fn heat_loss(&self, x: usize, y: usize) -> usize {
        self.heat_loss_map[y][x]
    }

    pub fn available_directions(&self, x: usize, y: usize, heading: Option<GearCityHeading>, minimum: usize, maximum: usize) -> impl Iterator<Item = GearCityDirection> + '_ {
        GearCityDirection::all()
            .filter(move |d| {
                (x != 0 || *d != GearCityDirection::West) &&
                    (x != self.width - 1 || *d != GearCityDirection::East) &&
                    (y != 0 || *d != GearCityDirection::North) &&
                    (y != self.height - 1 || *d != GearCityDirection::South) &&
                    !heading.is_some_and(|h| d.is_opposite(h.direction)) &&
                    !heading.is_some_and(|h| *d == h.direction && h.time >= maximum) &&
                    !heading.is_some_and(|h| *d != h.direction && h.time < minimum)
            })
    }
}

impl FromStr for GearCityGrid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let mut height = 0usize;
        let mut heat_loss_map: Vec<Vec<usize>> = Vec::new();

        for line in s.lines() {
            let heat_loss_row: Vec<_> = line.bytes()
                .map(|b| (b - 48) as usize)
                .collect();

            if width == 0 {
                width = heat_loss_row.len();
            }
            height += 1;
            heat_loss_map.push(heat_loss_row);
        }

        Ok(Self {
            width,
            height,
            heat_loss_map
        })
    }
}
