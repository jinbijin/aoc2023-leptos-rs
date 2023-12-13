#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolcanicPatchType {
    Ash,
    Rock
}

impl From<char> for VolcanicPatchType {
    fn from(value: char) -> Self {
        match value {
            '.' => VolcanicPatchType::Ash,
            '#' => VolcanicPatchType::Rock,
            _ => panic!("Invalid volcanic patch type")
        }
    }
}