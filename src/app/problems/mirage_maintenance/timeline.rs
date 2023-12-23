#[derive(Debug, Clone)]
pub struct Timeline(Vec<isize>);

impl From<&str> for Timeline {
    fn from(value: &str) -> Self {
        Self(value.split(' ').map(|x| x.parse::<isize>().unwrap()).collect())
    }
}

impl Timeline {
    fn get_differences(&self) -> Self {
        Self(self.0.windows(2).map(|w| { w[1] - w[0] })
            .collect())
    }

    fn is_non_zero_constant(&self) -> Option<isize> {
        if self.0.len() == 0 {
            None
        } else if self.0.iter().all(|x| *x == self.0[0]) {
            Some(self.0[0])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstabilityTimeline {
    timeline_stack: Vec<Timeline>,
    top_level_diff: isize,
}

impl InstabilityTimeline {
    pub fn new(timeline: Timeline) -> Self {
        let mut timeline_stack: Vec<Timeline> = Vec::with_capacity(timeline.0.len());
        timeline_stack.push(timeline);

        let mut top_level_diff = timeline_stack.last().unwrap().is_non_zero_constant();

        while top_level_diff == None {
            timeline_stack.push(timeline_stack.last().unwrap().get_differences());
            top_level_diff = timeline_stack.last().unwrap().is_non_zero_constant();
        }

        InstabilityTimeline {
            timeline_stack,
            top_level_diff: top_level_diff.unwrap()
        }
    }

    pub fn extrapolate(&self) -> isize {
        let mut diff = 0isize;

        for timeline in self.timeline_stack.iter().rev() {
            diff += timeline.0.last().unwrap();
        }

        diff
    }

    pub fn extrapolate_backwards(&self) -> isize {
        let mut diff = 0isize;

        for timeline in self.timeline_stack.iter().rev() {
            diff = timeline.0.first().unwrap() - diff;
        }

        diff
    }
}
