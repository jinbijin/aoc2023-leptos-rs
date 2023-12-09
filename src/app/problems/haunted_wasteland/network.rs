use std::collections::HashMap;
use super::{crossroads::Crossroads, direction::Direction, network_path::NetworkPath, node::Node};

pub struct Network(HashMap<Node, Crossroads>);

impl Network {
    pub fn traverse(&self, node: Node, dir: Direction) -> Node {
        let Network(map) = self;
        let crossroads = map.get(&node).unwrap();
        match dir {
            Direction::Left => crossroads.left,
            Direction::Right => crossroads.right,
        }
    }

    pub fn get_starting_nodes(&self) -> Vec<Node> {
        self.0.keys().filter_map(|key| if key.is_start() { Some(*key) } else { None }).collect()
    }

    pub fn get_path(&self, node: Node, dirs: &Vec<Direction>) -> NetworkPath {
        let len = dirs.len();
        let mut map: HashMap<(Node, usize), usize> = HashMap::new();
        let mut visited: Vec<Node> = Vec::new();
        let mut from = node;

        for (index, dir) in dirs.iter().cycle().enumerate() {
            if let Some(repeated_index) = map.get(&(from, index % len)) {
                // then we cycled, so visited is complete
                let (initial, repeating) = visited.split_at(*repeated_index);
                let initial = initial.to_vec();
                let repeating = repeating.to_vec();

                return NetworkPath::new(initial, repeating);
            }

            let dir = *dir;
            let to = self.traverse(from, dir);

            visited.push(from);
            map.insert((from, index % len), index);
            from = to;
        }

        unreachable!("Past infinite loop");
    }
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        for line in value.lines() {
            let (node, crossroads) = line.split_once(" = ").unwrap();
            map.insert(node.into(), crossroads.into());
        }

        Network(map)
    }
}
