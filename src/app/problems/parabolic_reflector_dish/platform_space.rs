#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformSpaceType {
    Round,
    Cube,
    Empty
}

impl From<char> for PlatformSpaceType {
    fn from(value: char) -> Self {
        match value {
            '.' => PlatformSpaceType::Empty,
            'O' => PlatformSpaceType::Round,
            '#' => PlatformSpaceType::Cube,
            _ => panic!("Invalid space type found")
        }
    }
}