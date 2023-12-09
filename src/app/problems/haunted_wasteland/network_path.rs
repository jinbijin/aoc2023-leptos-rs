use super::{eventually_periodic::EventuallyPeriodic, node::Node};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkPath {
    initial: Vec<Node>,
    repeating: Vec<Node>,
}

impl NetworkPath {
    pub fn new(initial: Vec<Node>, repeating: Vec<Node>) -> Self {
        Self {
            initial,
            repeating
        }
    }

    pub fn get_indices_satisfying(&self, predicate: fn(&Node) -> bool) -> EventuallyPeriodic {
        let initial_length = self.initial.len();
        let repeating_length = self.repeating.len();
        let initial_part = self.initial.iter().enumerate()
            .filter_map(|(i, x)| if predicate(x) { Some(i) } else { None })
            .collect::<Vec<usize>>();
        let repeating_part = self.repeating.iter().enumerate()
            .filter_map(|(i, x)| if predicate(x) { Some(i) } else { None })
            .collect::<Vec<usize>>();

        EventuallyPeriodic {
            initial_length,
            initial_part,
            repeating_length,
            repeating_part
        }
    }
}

