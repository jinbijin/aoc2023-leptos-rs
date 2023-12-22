use std::collections::HashMap;
use super::garden::Garden;

pub trait AsParallelUniverseWalker {
    fn as_parallel_universe_walker(&self) -> ParallelUniverseWalker<'_>;
    fn as_parallel_universe_walker_from(&self, start: (usize, usize)) -> ParallelUniverseWalker<'_>;
}

impl AsParallelUniverseWalker for Garden {
    fn as_parallel_universe_walker(&self) -> ParallelUniverseWalker<'_> {
        self.as_parallel_universe_walker_from(self.start())
    }

    fn as_parallel_universe_walker_from(&self, start: (usize, usize)) -> ParallelUniverseWalker<'_> {
        ParallelUniverseWalker {
            garden: self,
            plots: HashMap::from([(start, 0)]),
            distance: 0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParallelWalkResult {
    pub even_distance_count: usize,
    pub odd_distance_count: usize,
    pub steps_needed: usize,
}

pub struct ParallelUniverseWalker<'a> {
    garden: &'a Garden,
    plots: HashMap<(usize, usize), usize>,
    distance: usize,
}

impl<'a> ParallelUniverseWalker<'a> {
    pub fn possible_step_count_after(mut self, count: usize) -> usize {
        for _ in 0..count {
            self.step();
        }

        self.plots.iter()
            .filter(move |(_, d)| (**d % 2) == (count % 2))
            .count()
    }

    pub fn fill_plots(mut self) -> ParallelWalkResult {
        while self.step() { }

        let garden = self.garden;

        let even_distance_count =
            self.plots.iter()
                .filter(move |(plot, d)| garden.is_in_initial_grid(**plot) && (**d % 2) == 0)
                .count();
        let odd_distance_count =
            self.plots.iter()
                .filter(move |(plot, d)| garden.is_in_initial_grid(**plot) && (**d % 2) == 1)
                .count();
        let steps_needed =
            self.plots.iter()
                .filter_map(move |(plot, d)| if garden.is_in_initial_grid(*plot) { Some(*d) } else { None } )
                .max().unwrap();

        ParallelWalkResult {
            even_distance_count,
            odd_distance_count,
            steps_needed
        }
    }

    fn step(&mut self) -> bool {
        let mut inner_added = false;

        let distance = self.distance;
        let outer_plots: Vec<_> = {
            self.plots.iter()
                .filter_map(move |((x, y), d)| if *d == distance { Some((*x, *y)) } else { None })
                .collect()
        };

        self.distance += 1;
        let distance = self.distance;

        for outer_plot in outer_plots {
            let adjacent_plots = self.garden.adjacent_vertices(outer_plot);
            for adjacent_plot in adjacent_plots {
                if !self.plots.contains_key(&adjacent_plot) {
                    self.plots.insert(adjacent_plot, distance);
                    if self.garden.is_in_initial_grid(adjacent_plot) {
                        inner_added = true;
                    }
                }
            }
        }

        inner_added
    }
}
