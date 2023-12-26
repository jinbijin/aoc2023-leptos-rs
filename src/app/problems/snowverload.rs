#[cfg(feature = "ssr")]
mod component_config;

#[cfg(feature = "ssr")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "ssr")]
use self::component_config::{SnowComponent, SnowComponentConfig};
#[cfg(feature = "ssr")]
use crate::app::common::CycleResult;

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(_part: ProblemPart, input: String) -> usize {
        let config = SnowComponentConfig::from_str(&input);
        let graph = config.as_graph();
        let edges = config.edges();

        let mut increment_count = 0usize;

        let mut cycles: HashMap<(SnowComponent, SnowComponent), HashSet<(SnowComponent, SnowComponent)>> = HashMap::new();

        for i in 0..edges.len() {
            for j in (i+1)..edges.len() {
                // Clean up cycles
                let mut cleaned_cycles: HashMap<(SnowComponent, SnowComponent), HashSet<(SnowComponent, SnowComponent)>> = HashMap::new();
                for (key, value) in cycles.iter() {
                    if !value.contains(&(*edges[i].0, *edges[i].1)) && !value.contains(&(*edges[j].0, *edges[j].1)) && !cleaned_cycles.contains_key(key) {
                        let edges: Vec<_> = value.iter().collect();
                        for edge in edges {
                            cleaned_cycles.insert(*edge, value.clone());
                        }
                    }
                }

                cycles = cleaned_cycles;

                // Continue loop
                for k in (j+1)..edges.len() {
                    let has_existing_cycle = cycles.contains_key(&(*edges[k].0, *edges[k].1));

                    if !has_existing_cycle {
                        if increment_count % 100 == 0 {
                            logging::log!("Step {}: ({}, {}, {})", increment_count, i, j, k);
                        }
                        increment_count += 1;

                        match graph.splits_graph(edges[k], vec![edges[i], edges[j]]) {
                            CycleResult::Cycle(cycle) => {
                                let edges: Vec<_> = cycle.iter().collect();
                                for edge in edges {
                                    if !cycles.contains_key(edge) {
                                        cycles.insert(*edge, cycle.clone());
                                    }
                                }
                            },
                            CycleResult::Split(left, right) => {
                                return left * right;
                            }
                        }
                    }
                }
            }
        }

        0
    }
}