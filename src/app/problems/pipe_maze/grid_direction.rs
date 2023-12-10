#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridDirection {
    North,
    East,
    South,
    West
}

impl GridDirection {
    pub fn all() -> impl Iterator<Item = GridDirection> {
        AllGridDirectionsIterator { next: Some(GridDirection::North) }
    }

    pub fn opposite(self) -> Self {
        match self {
            GridDirection::North => GridDirection::South,
            GridDirection::East => GridDirection::West,
            GridDirection::South => GridDirection::North,
            GridDirection::West => GridDirection::East,
        }
    }
}

pub struct AllGridDirectionsIterator {
    next: Option<GridDirection>,
}

impl Iterator for AllGridDirectionsIterator {
    type Item = GridDirection;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next {
            let next = match current {
                GridDirection::North => Some(GridDirection::East),
                GridDirection::East => Some(GridDirection::South),
                GridDirection::South => Some(GridDirection::West),
                GridDirection::West => None
            };

            self.next = next;

            Some(current)
        } else {
            None
        }
    }
}
