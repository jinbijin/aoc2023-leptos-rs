use std::fmt::{Debug, Formatter};
use crate::app::common::AdjacencyHashGraph;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnowComponent<'a>(&'a str);

impl<'a> Debug for SnowComponent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnowComponentConfig<'a>(Vec<(SnowComponent<'a>, Vec<SnowComponent<'a>>)>);

impl<'a> SnowComponentConfig<'a> {
    pub fn from_str(s: &'a str) -> SnowComponentConfig<'a> {
        SnowComponentConfig(s.lines()
            .map(|line| {
                let (first, adjacent) = line.split_once(": ").unwrap();
                let first = SnowComponent(first);
                let adjacent: Vec<_> = adjacent.split(' ').map(|c| SnowComponent(c)).collect();

                (first, adjacent)
            })
            .collect())
    }

    pub fn edges(&self) -> Vec<(&SnowComponent<'a>, &SnowComponent<'a>)> {
        self.0.iter()
            .flat_map(|(from, adjacent)| {
                adjacent.iter().map(move |to| (from, to))
            })
            .collect()
    }

    pub fn as_graph(&self) -> AdjacencyHashGraph<SnowComponent<'a>> {
        let mut graph = AdjacencyHashGraph::new();

        for (from, adjacent) in self.0.iter() {
            graph.insert_vertex(*from);

            for to in adjacent.iter() {
                graph.insert_vertex(*to);
                graph.insert_edge(from, *to);
                graph.insert_edge(to, *from);
            }
        }

        graph
    }
}
