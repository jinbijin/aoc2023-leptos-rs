#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mirror {
    Horizontal(usize),
    Vertical(usize)
}

impl Mirror {
    pub fn range(width: usize, height: usize) -> impl Iterator<Item = Mirror> {
        (1..width).map(Mirror::Vertical)
            .chain((1..height).map(Mirror::Horizontal))
    }

    pub fn weight(&self) -> usize {
        match self {
            Mirror::Horizontal(lines_above) => 100 * (*lines_above),
            Mirror::Vertical(lines_left) => *lines_left,
        }
    }
}