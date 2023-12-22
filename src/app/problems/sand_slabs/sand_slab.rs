use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SandSlabRange {
    pub start: usize,
    pub end: usize,
}

impl SandSlabRange {
    pub fn single(value: usize) -> SandSlabRange {
        SandSlabRange {
            start: value,
            end: value
        }
    }

    pub fn infinite() -> SandSlabRange {
        SandSlabRange {
            start: 0,
            end: usize::MAX
        }
    }

    pub fn overlaps(&self, rhs: &SandSlabRange) -> bool {
        self.start <= rhs.end && rhs.start <= self.end
    }

    pub fn with_start(&self, start: usize) -> SandSlabRange {
        SandSlabRange {
            start,
            end: self.end - self.start + start
        }
    }
}

impl Debug for SandSlabRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}~{}", self.start, self.end)
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SandSlab {
    pub x: SandSlabRange,
    pub y: SandSlabRange,
    pub z: SandSlabRange,
}

impl SandSlab {
    pub fn infinite() -> SandSlab {
        SandSlab {
            x: SandSlabRange::infinite(),
            y: SandSlabRange::infinite(),
            z: SandSlabRange::single(0)
        }
    }

    pub fn overlaps(&self, rhs: &SandSlab) -> bool {
        self.x.overlaps(&rhs.x) && self.y.overlaps(&rhs.y)
    }

    pub fn with_z_start(&self, z_start: usize) -> SandSlab {
        SandSlab {
            x: self.x,
            y: self.y,
            z: self.z.with_start(z_start)
        }
    }
}

impl Debug for SandSlab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?},{:?}]", self.x, self.y, self.z)
    }
}

impl FromStr for SandSlab {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let start: Vec<_> = start.splitn(3, ',').map(|s| s.parse::<usize>().unwrap()).collect();
        let end: Vec<_> = end.splitn(3, ',').map(|s| s.parse::<usize>().unwrap()).collect();

        Ok(SandSlab {
            x: SandSlabRange {
                start: start[0],
                end: end[0],
            },
            y: SandSlabRange {
                start: start[1],
                end: end[1]
            },
            z: SandSlabRange {
                start: start[2],
                end: end[2]
            }
        })
    }
}

pub struct SandSlabSnapshot{
    slab_bottoms: BTreeMap<usize, Vec<SandSlab>>,
    height: usize
}

impl SandSlabSnapshot {
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Deref for SandSlabSnapshot {
    type Target = BTreeMap<usize, Vec<SandSlab>>;

    fn deref(&self) -> &Self::Target {
        &self.slab_bottoms
    }
}

impl FromStr for SandSlabSnapshot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slab_bottoms: BTreeMap<usize, Vec<SandSlab>> = BTreeMap::new();
        let mut height = 0usize;

        for line in s.lines() {
            let slab = line.parse::<SandSlab>().unwrap();

            if slab.z.end > height {
                height = slab.z.end;
            }

            if let Some(slabs) = slab_bottoms.get_mut(&slab.z.start) {
                slabs.push(slab);
            } else {
                slab_bottoms.insert(slab.z.start, vec![slab]);
            }
        }

        Ok(SandSlabSnapshot{
            slab_bottoms,
            height
        })
    }
}
