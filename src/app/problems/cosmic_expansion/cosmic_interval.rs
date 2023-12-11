#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CosmicInterval {
    start: usize,
    end: usize,
    before_count: usize,
    after_count: usize
}

impl CosmicInterval {
    pub fn weight(&self, expansion_factor: usize) -> usize {
        let distance = 1 + expansion_factor * (self.end - self.start - 1);
        distance * self.before_count * self.after_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosmicIntervals(Vec<CosmicInterval>);

impl CosmicIntervals {
    pub fn from_counts(galaxy_counts: &Vec<usize>) -> CosmicIntervals {
        let count = galaxy_counts.iter().sum();

        let mut start = 0usize;
        let mut end = 0usize;
        let mut before_count = 0usize;
        let mut after_count = count;

        let mut intervals: Vec<CosmicInterval> = Vec::new();

        for count in galaxy_counts.iter() {
            if *count == 0 {
                end += 1;
            } else {
                if before_count != 0 && after_count != 0 {
                    intervals.push(CosmicInterval { start, end, before_count, after_count });
                }
                start = end;
                end += 1;
                before_count += count;
                after_count -= count;
            }
        }

        CosmicIntervals(intervals)
    }

    pub fn weight(&self, expansion_factor: usize) -> usize {
        self.0.iter().map(|interval| interval.weight(expansion_factor)).sum()
    }
}
