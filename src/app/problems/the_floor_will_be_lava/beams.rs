#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamSegment {
    Vertical {
        x: usize,
        y_from: Option<usize>,
        y_to: Option<usize>,
    },
    Horizontal {
        x_from: Option<usize>,
        x_to: Option<usize>,
        y: usize,
    }
}

impl BeamSegment {
    pub fn intersects(&self, other: &BeamSegment) -> bool {
        match self {
            BeamSegment::Horizontal { y, x_from, x_to} => {
                if let BeamSegment::Vertical { x, y_from, y_to } = other {
                    !x_from.is_some_and(|x_from| x_from >= *x) && !x_to.is_some_and(|x_to| x_to <= *x) &&
                        !y_from.is_some_and(|y_from| y_from >= *y) && !y_to.is_some_and(|y_to| y_to <= *y)
                } else {
                    false
                }
            },
            BeamSegment::Vertical { x, y_from, y_to } => {
                if let BeamSegment::Horizontal { y, x_from, x_to} = other {
                    !x_from.is_some_and(|x_from| x_from >= *x) && !x_to.is_some_and(|x_to| x_to <= *x) &&
                        !y_from.is_some_and(|y_from| y_from >= *y) && !y_to.is_some_and(|y_to| y_to <= *y)
                } else {
                    false
                }
            }
        }
    }
}
