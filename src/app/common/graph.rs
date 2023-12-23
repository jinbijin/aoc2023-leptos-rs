use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

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