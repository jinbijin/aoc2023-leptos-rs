use super::grid_direction::GridDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pipe{
    Some([GridDirection; 2]),
    None,
    Start
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Some([GridDirection::North, GridDirection::South]),
            '-' => Pipe::Some([GridDirection::East, GridDirection::West]),
            'L' => Pipe::Some([GridDirection::North, GridDirection::East]),
            'J' => Pipe::Some([GridDirection::North, GridDirection::West]),
            '7' => Pipe::Some([GridDirection::South, GridDirection::West]),
            'F' => Pipe::Some([GridDirection::South, GridDirection::East]),
            '.' => Pipe::None,
            'S' => Pipe::Start,
            _ => panic!("unexpected character for pipe")
        }
    }
}
