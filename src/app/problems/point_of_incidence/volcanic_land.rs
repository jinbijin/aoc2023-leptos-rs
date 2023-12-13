use super::{mirror::Mirror,volcanic_patch::VolcanicPatchType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VolcanicStrip(Vec<VolcanicPatchType>);

impl VolcanicStrip {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, patch_type: VolcanicPatchType) {
        self.0.push(patch_type);
    }

    fn difference(&self, other: &VolcanicStrip) -> usize {
        self.0.iter()
            .zip(other.0.iter())
            .filter(|(x, y)| x != y)
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VolcanicLand {
    horizontal_strips: Vec<VolcanicStrip>,
    vertical_strips: Vec<VolcanicStrip>
}

impl VolcanicLand {
    pub fn find_mirror(&self) -> Mirror {
        Mirror::range(self.get_width(), self.get_height())
            .find(|mirror| self.is_mirror(*mirror))
            .unwrap()
    }

    pub fn find_smudged_mirror(&self) -> Mirror {
        Mirror::range(self.get_width(), self.get_height())
            .find(|mirror| self.count_smudges(*mirror) == 1)
            .unwrap()
    }

    fn is_mirror(&self, mirror: Mirror) -> bool {
        let width = self.get_width();
        let height = self.get_height();

        match mirror {
            Mirror::Vertical(cols_left) =>
                (0..cols_left).rev()
                    .zip(cols_left..width)
                    .all(|(i, j)| self.vertical_strips[i] == self.vertical_strips[j]),
            Mirror::Horizontal(rows_above) =>
                (0..rows_above).rev()
                    .zip(rows_above..height)
                    .all(|(i, j)| self.horizontal_strips[i] == self.horizontal_strips[j]),
        }
    }

    fn count_smudges(&self, mirror: Mirror) -> usize {
        let width = self.get_width();
        let height = self.get_height();

        match mirror {
            Mirror::Vertical(cols_left) =>
                (0..cols_left).rev()
                    .zip(cols_left..width)
                    .map(|(i, j)| self.vertical_strips[i].difference(&self.vertical_strips[j]))
                    .sum(),
            Mirror::Horizontal(rows_above) =>
                (0..rows_above).rev()
                    .zip(rows_above..height)
                    .map(|(i, j)| self.horizontal_strips[i].difference(&self.horizontal_strips[j]))
                    .sum(),
        }
    }

    fn get_width(&self) -> usize {
        self.vertical_strips.len()
    }

    fn get_height(&self) -> usize {
        self.horizontal_strips.len()
    }
}

impl From<&str> for VolcanicLand {
    fn from(value: &str) -> Self {
        let mut horizontal_strips: Vec<VolcanicStrip> = Vec::new();
        let mut vertical_strips: Vec<VolcanicStrip> = Vec::new();

        for (y, line) in value.lines().enumerate() {
            horizontal_strips.push(VolcanicStrip::new());

            for (x, patch_type) in line.chars().map(|c| -> VolcanicPatchType { c.into() }).enumerate() {
                if y == 0 {
                    vertical_strips.push(VolcanicStrip::new());
                }

                horizontal_strips[y].push(patch_type);
                vertical_strips[x].push(patch_type);
            }
        }

        VolcanicLand {
            horizontal_strips,
            vertical_strips
        }
    }
}
