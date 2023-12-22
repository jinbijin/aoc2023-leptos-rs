use std::collections::{BTreeMap, HashMap, HashSet};
use super::sand_slab::{SandSlab, SandSlabSnapshot};

pub trait AsSandStack {
    fn as_sand_stack(&self) -> SandStack;
}

impl AsSandStack for SandSlabSnapshot {
    fn as_sand_stack(&self) -> SandStack {
        let mut builder = SandStackBuilder::with_capacity(self.len(), self.height());

        for slabs in self.values() {
            for slab in slabs {
                builder.drop_into(slab);
            }
        }

        builder.into()
    }
}

#[derive(Debug)]
pub struct SandStack {
    supporting: HashMap<SandSlab, Vec<SandSlab>>,
    supported_by: HashMap<SandSlab, Vec<SandSlab>>,
}

impl SandStack {
    pub fn disintegrateable_slab_count(&self) -> usize {
        self.supporting.values()
            .filter(|supporting| {
                supporting.iter().all(|s| self.supported_by.get(s).unwrap().len() > 1)
            })
            .count()
    }

    pub fn load_bearing_score(&self) -> usize {
        self.supporting.keys()
            .map(|s| self.load_on(s))
            .sum()
    }

    fn load_on(&self, slab: &SandSlab) -> usize {
        let mut load = 0usize;
        let mut slabs: BTreeMap<usize, HashSet<SandSlab>> = BTreeMap::from([(slab.z.end, HashSet::from([*slab]))]);

        while let Some((_, supporting)) = slabs.pop_first() {
            let supporting = &supporting;
            let supported_slabs: HashSet<_> = supporting.iter()
                .flat_map(|s| self.supporting.get(&s).unwrap().iter())
                .collect();

            for supported_slab in supported_slabs {
                if self.supported_by.get(supported_slab).unwrap().iter().all(|s| supporting.contains(s)) {
                    load += 1;
                    if let Some(set) = slabs.get_mut(&supported_slab.z.end) {
                        set.insert(*supported_slab);
                    } else {
                        slabs.insert(supported_slab.z.end, HashSet::from([*supported_slab]));
                    }
                }
            }
        }

        load
    }
}

impl From<SandStackBuilder> for SandStack {
    fn from(value: SandStackBuilder) -> Self {
        SandStack {
            supporting: value.supporting,
            supported_by: value.supported_by
        }
    }
}

struct SandStackBuilder {
    supporting: HashMap<SandSlab, Vec<SandSlab>>,
    supported_by: HashMap<SandSlab, Vec<SandSlab>>,
    slab_tops: Vec<Vec<SandSlab>>,
}

impl SandStackBuilder {
    fn with_capacity(slab_count: usize, max_height: usize) -> SandStackBuilder {
        // Allocate and fill up front
        let mut slab_tops = vec![vec![]; max_height];
        slab_tops[0].push(SandSlab::infinite());

        SandStackBuilder {
            supporting: HashMap::with_capacity(slab_count),
            supported_by: HashMap::with_capacity(slab_count),
            slab_tops
        }
    }

    fn drop_into(&mut self, slab: &SandSlab) {
        let (top, supported_by) = self.slab_tops.iter().enumerate().rev()
            .filter_map(|(i, slabs)| {
                let overlapping_slabs: Vec<_> = slabs.iter()
                    .filter(move |s| s.overlaps(slab))
                    .map(|s| *s)
                    .collect();
                if overlapping_slabs.len() > 0 {
                    Some((i, overlapping_slabs))
                } else {
                    None
                }
            })
            .next().unwrap(); // Unwrap is safe because we put a virtual infinite slab at the bottom of the stack

        let slab = slab.with_z_start(top + 1);
        self.supporting.insert(slab, vec![]);

        if top > 0 {
            for supporting_slab in supported_by.iter() {
                self.supporting.get_mut(supporting_slab).unwrap().push(slab); // Unwrap is safe because of the insert above
            }
            self.supported_by.insert(slab, supported_by);
        }

        self.slab_tops[slab.z.end].push(slab);
    }
}
