use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Garden {
    width: usize,
    height: usize,
    start: (usize, usize),
    rocks: HashSet<(usize, usize)>,
}

impl Garden {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    pub fn north_plot(&self) -> (usize, usize) {
        (self.start.0, 0)
    }

    pub fn south_plot(&self) -> (usize, usize) {
        (self.start.0, self.height - 1)
    }

    pub fn west_plot(&self) -> (usize, usize) {
        (0, self.start.1)
    }

    pub fn east_plot(&self) -> (usize, usize) {
        (self.width - 1, self.start.1)
    }

    pub fn north_west_plot(&self) -> (usize, usize) {
        (0, 0)
    }

    pub fn north_east_plot(&self) -> (usize, usize) {
        (self.width - 1, 0)
    }

    pub fn south_west_plot(&self) -> (usize, usize) {
        (0, self.height - 1)
    }

    pub fn south_east_plot(&self) -> (usize, usize) {
        (self.width - 1, self.height - 1)
    }

    pub fn is_in_initial_grid(&self, coords: (usize, usize)) -> bool {
        let (x, y) = coords;
        x < self.width && y < self.height
    }

    pub fn adjacent_vertices(&self, coords: (usize, usize)) -> Vec<(usize, usize)>{
        let mut result: Vec<(usize, usize)> = Vec::with_capacity(4);
        let (x, y) = coords;

        if x > 0 && !self.rocks.contains(&(x - 1, y)) {
            result.push((x - 1, y));
        }
        if y > 0 && !self.rocks.contains(&(x, y - 1)) {
            result.push((x, y - 1));
        }
        if x < self.width - 1 && !self.rocks.contains(&(x + 1, y)) {
            result.push((x + 1, y));
        }
        if y < self.height - 1 && !self.rocks.contains(&(x, y + 1)) {
            result.push((x, y + 1));
        }

        result
    }
}

impl FromStr for Garden {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let mut height = 0usize;
        let mut start: Option<(usize, usize)> = None;
        let mut rocks: HashSet<(usize, usize)> = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            height += 1;
            for (x, c) in line.chars().enumerate() {
                if y == 0 {
                    width += 1;
                }

                match c {
                    '#' => {
                        rocks.insert((x, y));
                    },
                    'S' => {
                        start = Some((x, y));
                    },
                    _ => (),
                }
            }
        }

        Ok(Self{
            width,
            height,
            start: start.unwrap(),
            rocks
        })
    }
}
