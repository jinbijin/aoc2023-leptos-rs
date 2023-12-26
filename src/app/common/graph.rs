use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter::once;

#[derive(Clone)]
pub struct DistanceHashGraph<Vertex>(HashMap<Vertex, Vec<(usize, Vertex)>>);

impl<Vertex> DistanceHashGraph<Vertex> {
    pub fn new() -> DistanceHashGraph<Vertex> {
        DistanceHashGraph(HashMap::new())
    }
}

impl<Vertex: Eq + Hash> DistanceHashGraph<Vertex> {
    pub fn longest_distance_between(&self, from: Vertex, to: Vertex) -> Option<usize> {
        let mut results: BinaryHeap<usize> = BinaryHeap::new();
        let mut state_stack: Vec<(usize, HashSet<&Vertex>, &Vertex)> = vec![(0, HashSet::from([&from]), &from)];

        while let Some((distance, visited, vertex)) = state_stack.pop() {
            if let Some(next_vertices) = self.0.get(vertex) {
                for (next_distance, next_vertex) in next_vertices.iter() {
                    let distance = distance + *next_distance;
                    if next_vertex == &to {
                        results.push(distance);
                    } else if !visited.contains(&next_vertex) {
                        let mut next_visited = visited.clone();
                        next_visited.insert(next_vertex);

                        state_stack.push((distance, next_visited, next_vertex));
                    }
                }
            }
        }

        results.pop()
    }

    /// Inserts a vertex into the graph
    pub fn insert_vertex(&mut self, vertex: Vertex) {
        if !self.0.contains_key(&vertex) {
            self.0.insert(vertex, vec![]);
        }
    }

    /// Inserts an edge into the graph
    ///
    /// Panics if `from` is not in graph
    pub fn insert_edge(&mut self, from: &Vertex, to: Vertex, distance: usize) {
        self.0.get_mut(from).unwrap().push((distance, to));
    }
}

impl<Vertex: Debug + Hash> Debug for DistanceHashGraph<Vertex> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vertex_count = self.0.len();
        let edge_count: usize = self.0.values().map(|x| x.len()).sum();

        writeln!(f, "Graph with {} vertices and {} edges", vertex_count, edge_count)?;

        for (vertex, edges) in self.0.iter() {
            writeln!(f, "{:?}:", vertex)?;

            for (distance, destination) in edges.iter() {
                writeln!(f, "-({})-> {:?}", distance, destination)?;
            }
        }

        Ok(())
    }
}

pub struct AdjacencyHashGraph<Vertex>(HashMap<Vertex, HashSet<Vertex>>);

impl<Vertex> AdjacencyHashGraph<Vertex> {
    pub fn new() -> AdjacencyHashGraph<Vertex> {
        AdjacencyHashGraph(HashMap::new())
    }
}

impl<Vertex: Clone + Eq + Hash> AdjacencyHashGraph<Vertex> {
    // well, assuming the number is at most two for now
    pub fn splits_graph(&self, edge: (&Vertex, &Vertex), forbidden_edges: Vec<(&Vertex, &Vertex)>) -> CycleResult<Vertex> {
        let mut forbidden_edges: HashSet<_> = forbidden_edges.into_iter()
            .flat_map(|(from, to)| once((from, to)).chain(once((to, from))))
            .collect();

        let (left, right) = edge;
        forbidden_edges.insert((left, right));
        forbidden_edges.insert((right, left));

        let mut left_visited: HashMap<&Vertex, Vec<&Vertex>> = HashMap::from([(left, vec![left])]);
        let mut left_stack: Vec<(&Vertex, Vec<&Vertex>)> = vec![(left, vec![left])];
        let mut right_visited: HashMap<&Vertex, Vec<&Vertex>> = HashMap::from([(right, vec![right])]);
        let mut right_stack: Vec<(&Vertex, Vec<&Vertex>)> = vec![(right, vec![right])];

        while left_stack.len() > 0 || right_stack.len() > 0 {
            if let Some((current_left, left_cycle_stack)) = left_stack.pop() {
                for adjacent in self.0.get(current_left).unwrap() {
                    if !forbidden_edges.contains(&(current_left, adjacent)) {
                        if let Some(right_cycle_stack) = right_visited.get(adjacent) {
                            let mut cycle: HashSet<(Vertex, Vertex)> = HashSet::new();

                            // Add original edge
                            cycle.insert((left.clone(), right.clone()));
                            cycle.insert((right.clone(), left.clone()));

                            // Add left edges
                            for i in 0..(left_cycle_stack.len() - 1) {
                                cycle.insert((left_cycle_stack[i].clone(), left_cycle_stack[i + 1].clone()));
                                cycle.insert((left_cycle_stack[i + 1].clone(), left_cycle_stack[i].clone()));
                            }

                            // Add right edges
                            for i in 0..(right_cycle_stack.len() - 1) {
                                cycle.insert((right_cycle_stack[i].clone(), right_cycle_stack[i + 1].clone()));
                                cycle.insert((right_cycle_stack[i + 1].clone(), right_cycle_stack[i].clone()));
                            }

                            // Add new edge
                            cycle.insert(((*left_cycle_stack.last().unwrap()).clone(), (*right_cycle_stack.last().unwrap()).clone()));
                            cycle.insert(((*right_cycle_stack.last().unwrap()).clone(), (*left_cycle_stack.last().unwrap()).clone()));

                            return CycleResult::Cycle(cycle);
                        }

                        if !left_visited.contains_key(adjacent) {
                            let mut left_cycle_stack = left_cycle_stack.clone();
                            left_cycle_stack.push(adjacent);

                            left_visited.insert(adjacent, left_cycle_stack.clone());
                            left_stack.push((adjacent, left_cycle_stack));
                        }
                    }
                }
            }

            if let Some((current_right, right_cycle_stack)) = right_stack.pop() {
                for adjacent in self.0.get(current_right).unwrap() {
                    if !forbidden_edges.contains(&(current_right, adjacent)) {
                        if let Some(left_cycle_stack) = left_visited.get(adjacent) {
                            let mut cycle: HashSet<(Vertex, Vertex)> = HashSet::new();

                            // Add original edge
                            cycle.insert((left.clone(), right.clone()));
                            cycle.insert((right.clone(), left.clone()));

                            // Add left edges
                            for i in 0..(left_cycle_stack.len() - 1) {
                                cycle.insert((left_cycle_stack[i].clone(), left_cycle_stack[i + 1].clone()));
                                cycle.insert((left_cycle_stack[i + 1].clone(), left_cycle_stack[i].clone()));
                            }

                            // Add right edges
                            for i in 0..(right_cycle_stack.len() - 1) {
                                cycle.insert((right_cycle_stack[i].clone(), right_cycle_stack[i + 1].clone()));
                                cycle.insert((right_cycle_stack[i + 1].clone(), right_cycle_stack[i].clone()));
                            }

                            // Add new edge
                            cycle.insert(((*left_cycle_stack.last().unwrap()).clone(), (*right_cycle_stack.last().unwrap()).clone()));
                            cycle.insert(((*right_cycle_stack.last().unwrap()).clone(), (*left_cycle_stack.last().unwrap()).clone()));

                            return CycleResult::Cycle(cycle);
                        }

                        if !right_visited.contains_key(adjacent) {
                            let mut right_cycle_stack = right_cycle_stack.clone();
                            right_cycle_stack.push(adjacent);

                            right_visited.insert(adjacent, right_cycle_stack.clone());
                            right_stack.push((adjacent, right_cycle_stack));
                        }
                    }
                }
            }
        }

        CycleResult::Split(left_visited.len(), right_visited.len())
    }

}

impl<Vertex: Eq + Hash> AdjacencyHashGraph<Vertex> {
    /// Inserts a vertex into the graph
    pub fn insert_vertex(&mut self, vertex: Vertex) {
        if !self.0.contains_key(&vertex) {
            self.0.insert(vertex, HashSet::new());
        }
    }

    /// Inserts an edge into the graph
    ///
    /// Panics if `from` is not in graph
    pub fn insert_edge(&mut self, from: &Vertex, to: Vertex) {
        self.0.get_mut(from).unwrap().insert(to);
    }
}

impl<Vertex: Debug + Hash> Debug for AdjacencyHashGraph<Vertex> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vertex_count = self.0.len();
        let edge_count: usize = self.0.values().map(|x| x.len()).sum();
        let out_degree: usize = self.0.values().map(|x| x.len()).min().unwrap();

        writeln!(f, "Graph (out degree {}) with {} vertices and {} edges", out_degree, vertex_count, edge_count)?;

        for (vertex, edges) in self.0.iter() {
            write!(f, "\n{:?} -->", vertex)?;

            for destination in edges.iter() {
                write!(f, " {:?}", destination)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CycleResult<Vertex> {
    Split(usize, usize),
    Cycle(HashSet<(Vertex, Vertex)>)
}
