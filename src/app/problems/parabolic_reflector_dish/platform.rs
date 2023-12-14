use std::fmt::{Debug, Formatter};
use super::platform_space::PlatformSpaceType;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PlatformStrip(Vec<PlatformSpaceType>);

impl PlatformStrip {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, space_type: PlatformSpaceType) {
        self.0.push(space_type);
    }

    fn tilt(&self) -> Self {
        let mut spaces: Vec<PlatformSpaceType> = Vec::new();

        for (i, space_type) in self.0.iter().enumerate() {
            match space_type {
                PlatformSpaceType::Empty => {},
                PlatformSpaceType::Round => {
                    spaces.push(PlatformSpaceType::Round);
                },
                PlatformSpaceType::Cube => {
                    while spaces.len() < i {
                        spaces.push(PlatformSpaceType::Empty);
                    }
                    spaces.push(PlatformSpaceType::Cube);
                }
            }
        }

        while spaces.len() < self.0.len() {
            spaces.push(PlatformSpaceType::Empty);
        }

        PlatformStrip(spaces)
    }

    fn tilt_back(&self) -> Self {
        let mut spaces: Vec<PlatformSpaceType> = Vec::new();

        for (i, space_type) in self.0.iter().rev().enumerate() {
            match space_type {
                PlatformSpaceType::Empty => {},
                PlatformSpaceType::Round => {
                    spaces.push(PlatformSpaceType::Round);
                },
                PlatformSpaceType::Cube => {
                    while spaces.len() < i {
                        spaces.push(PlatformSpaceType::Empty);
                    }
                    spaces.push(PlatformSpaceType::Cube);
                }
            }
        }

        while spaces.len() < self.0.len() {
            spaces.push(PlatformSpaceType::Empty);
        }

        spaces.reverse();

        PlatformStrip(spaces)
    }

    fn get_unshifted_load(&self) -> usize {
        let max_weight = self.0.len();
        self.0.iter().enumerate()
            .filter_map(|(i, space_type)| {
                if *space_type == PlatformSpaceType::Round {
                    Some(max_weight - i)
                } else {
                    None
                }
            })
            .sum()
    }

    fn get_total_load(&self) -> usize {
        let mut total_load = 0usize;
        let mut weight = self.0.len();

        for (i, space_type) in self.0.iter().enumerate() {
            match space_type {
                PlatformSpaceType::Empty => {},
                PlatformSpaceType::Round => {
                    total_load += weight;
                    weight -= 1;
                },
                PlatformSpaceType::Cube => {
                    weight = self.0.len() - i - 1;
                },
            }
        }

        total_load
    }
}

impl Debug for PlatformStrip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in self.0.iter() {
            match x {
                PlatformSpaceType::Empty => write!(f, ".")?,
                PlatformSpaceType::Cube => write!(f, "#")?,
                PlatformSpaceType::Round => write!(f, "O")?
            }
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    horizontal_strips: Vec<PlatformStrip>,
    vertical_strips: Vec<PlatformStrip>
}

impl Platform {
    pub fn get_unshifted_load(&self) -> usize {
        self.vertical_strips.iter().map(|strip| strip.get_unshifted_load()).sum()
    }

    pub fn get_total_load(&self) -> usize {
        self.vertical_strips.iter().map(|strip| strip.get_total_load()).sum()
    }

    pub fn cycle(&self) -> Self {
        self.tilt_north()
            .tilt_west()
            .tilt_south()
            .tilt_east()
    }

    fn tilt_north(&self) -> Self {
        let vertical_strips: Vec<_> = self.vertical_strips.iter()
            .map(|strip| strip.tilt())
            .collect();
        let horizontal_strips: Vec<_> = (0..self.horizontal_strips.len())
            .map(|y| -> Vec<_> {
                (0..self.vertical_strips.len())
                    .map(|x| vertical_strips[x].0[y])
                    .collect()
            })
            .map(|spaces| PlatformStrip(spaces))
            .collect();

        Self {
            horizontal_strips,
            vertical_strips
        }
    }

    fn tilt_west(&self) -> Self {
        let horizontal_strips: Vec<_> = self.horizontal_strips.iter()
            .map(|strip| strip.tilt())
            .collect();
        let vertical_strips: Vec<_> = (0..self.vertical_strips.len())
            .map(|x| -> Vec<_> {
                (0..self.horizontal_strips.len())
                    .map(|y| horizontal_strips[y].0[x])
                    .collect()
            })
            .map(|spaces| PlatformStrip(spaces))
            .collect();

        Self {
            horizontal_strips,
            vertical_strips
        }
    }

    fn tilt_south(&self) -> Self {
        let vertical_strips: Vec<_> = self.vertical_strips.iter()
            .map(|strip| strip.tilt_back())
            .collect();
        let horizontal_strips: Vec<_> = (0..self.horizontal_strips.len())
            .map(|y| -> Vec<_> {
                (0..self.vertical_strips.len())
                    .map(|x| vertical_strips[x].0[y])
                    .collect()
            })
            .map(|spaces| PlatformStrip(spaces))
            .collect();

        Self {
            horizontal_strips,
            vertical_strips
        }
    }

    fn tilt_east(&self) -> Self {
        let horizontal_strips: Vec<_> = self.horizontal_strips.iter()
            .map(|strip| strip.tilt_back())
            .collect();
        let vertical_strips: Vec<_> = (0..self.vertical_strips.len())
            .map(|x| -> Vec<_> {
                (0..self.horizontal_strips.len())
                    .map(|y| horizontal_strips[y].0[x])
                    .collect()
            })
            .map(|spaces| PlatformStrip(spaces))
            .collect();

        Self {
            horizontal_strips,
            vertical_strips
        }
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for strip in self.horizontal_strips.iter() {
            writeln!(f, "{:?}", strip)?;
        }

        Ok(())
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let mut horizontal_strips: Vec<PlatformStrip> = Vec::new();
        let mut vertical_strips: Vec<PlatformStrip> = Vec::new();

        for (y, line) in value.lines().enumerate() {
            horizontal_strips.push(PlatformStrip::new());

            for (x, patch_type) in line.chars().map(|c| -> PlatformSpaceType { c.into() }).enumerate() {
                if y == 0 {
                    vertical_strips.push(PlatformStrip::new());
                }

                vertical_strips[x].push(patch_type);
                horizontal_strips[y].push(patch_type);
            }
        }

        Platform {
            horizontal_strips,
            vertical_strips
        }
    }
}
