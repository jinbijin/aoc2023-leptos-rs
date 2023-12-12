#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpringCondition {
    Damaged,
    Operational
}

impl SpringCondition {
    pub fn read(c: char) -> Option<SpringCondition> {
        match c {
            '#' => Some(SpringCondition::Damaged),
            '.' => Some(SpringCondition::Operational),
            '?' => None,
            _ => panic!("Invalid character for spring condition")
        }
    }
}
