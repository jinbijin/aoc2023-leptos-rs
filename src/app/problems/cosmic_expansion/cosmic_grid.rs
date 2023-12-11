use super::{cosmic_coords::CosmicCoords, cosmic_interval::CosmicIntervals};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosmicGrid {
    galaxies: Vec<CosmicCoords>,
    with_x_count: Vec<usize>,
    with_y_count: Vec<usize>,
}

impl CosmicGrid {
    pub fn weight(&self, expansion_factor: usize) -> usize {
        let x_weight = CosmicIntervals::from_counts(&self.with_x_count).weight(expansion_factor);
        let y_weight = CosmicIntervals::from_counts(&self.with_y_count).weight(expansion_factor);
        x_weight + y_weight
    }
}

impl From<&str> for CosmicGrid {
    fn from(value: &str) -> Self {
        let mut galaxies: Vec<CosmicCoords> = Vec::new();
        let mut with_x_count: Vec<usize> = Vec::new();
        let mut with_y_count: Vec<usize> = Vec::new();

        for (y, line) in value.lines().enumerate() {
            with_y_count.push(0);

            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    with_x_count.push(0);
                }
                if c == '#' {
                    galaxies.push(CosmicCoords { x, y });
                    with_x_count[x] += 1;
                    with_y_count[y] += 1;
                }
            }
        }

        CosmicGrid {
            galaxies,
            with_x_count,
            with_y_count
        }
    }
}