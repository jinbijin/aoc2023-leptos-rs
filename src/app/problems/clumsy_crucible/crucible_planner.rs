use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use super::gear_city_grid::{GearCityDirection, GearCityGrid, GearCityHeading};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrucibleConfig {
    minimum: usize,
    maximum: usize,
}

impl CrucibleConfig {
    pub fn legacy() -> CrucibleConfig {
        CrucibleConfig {
            minimum: 0,
            maximum: 3,
        }
    }

    pub fn ultra() -> CrucibleConfig {
        CrucibleConfig {
            minimum: 4,
            maximum: 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CrucibleLocation {
    // Needs to be at top for derived Ord to be correct
    heat_lost: usize,
    x: usize,
    y: usize,
    heading: Option<GearCityHeading>
}

impl CrucibleLocation {
    fn new() -> CrucibleLocation {
        CrucibleLocation {
            heat_lost: 0,
            x: 0,
            y: 0,
            heading: None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CrucibleVisitedLocation {
    x: usize,
    y: usize,
    direction: GearCityDirection
}

impl From<CrucibleLocation> for CrucibleVisitedLocation {
    fn from(value: CrucibleLocation) -> Self {
        Self {
            x: value.x,
            y: value.y,
            direction: value.heading.unwrap().direction()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HeatLossStats {
    heat_lost: usize,
    time_in_direction: usize,
}

#[derive(Debug)]
pub struct CruciblePlanner<'a> {
    grid: &'a GearCityGrid,
    config: CrucibleConfig,
    locations: BinaryHeap<Reverse<CrucibleLocation>>,
    location_set: HashSet<CrucibleLocation>,
    visited_locations: HashMap<CrucibleVisitedLocation, Vec<HeatLossStats>>,
}

impl<'a> CruciblePlanner<'a> {
    pub fn from(grid: &'a GearCityGrid, config: CrucibleConfig) -> CruciblePlanner<'a> {
        CruciblePlanner {
            grid,
            config,
            locations: BinaryHeap::from([Reverse(CrucibleLocation::new())]),
            location_set: HashSet::from([CrucibleLocation::new()]),
            visited_locations: HashMap::new(),
        }
    }

    pub fn minimum_heat_loss(mut self) -> usize {
        loop {
            if let Some(heat_lost) = self.step() {
                return heat_lost;
            }
        }
    }

    fn step(&mut self) -> Option<usize> {
        if let Some(Reverse(location)) = self.locations.pop() {
            self.location_set.remove(&location);
            let mut push_to_heap = true;

            if let Some(heading) = location.heading {
                if heading.time_in_direction() >= self.config.minimum {
                    let visited_location: CrucibleVisitedLocation = location.into();
                    if let Some(stats) = self.visited_locations.get_mut(&visited_location) {
                        if stats.iter().any(|s| s.time_in_direction <= heading.time_in_direction()) {
                            push_to_heap = false;
                        }
                        else {
                            stats.push(HeatLossStats { heat_lost: location.heat_lost, time_in_direction: heading.time_in_direction() });
                        }
                    } else {
                        self.visited_locations.insert(visited_location, vec![HeatLossStats { heat_lost: location.heat_lost, time_in_direction: heading.time_in_direction() }]);
                    }
                }
            }

            if push_to_heap {
                for direction in self.grid.available_directions(location.x, location.y, location.heading, self.config.minimum, self.config.maximum) {
                    let (x, y) = direction.shift(location.x, location.y);
                    let heading = location.heading.map_or(GearCityHeading::from(direction), |h| h.with(direction));
                    let heat_lost = location.heat_lost + self.grid.heat_loss(x, y);

                    if self.grid.is_endpoint(x, y) && heading.time_in_direction() >= self.config.minimum {
                        return Some(heat_lost);
                    }

                    let new_location = CrucibleLocation {
                        heat_lost,
                        x,
                        y,
                        heading: Some(heading)
                    };

                    if self.location_set.insert(new_location) {
                        self.locations.push(Reverse(new_location));
                    }
                }
            }
        }

        None
    }
}
