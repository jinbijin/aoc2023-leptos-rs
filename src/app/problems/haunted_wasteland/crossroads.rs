use super::node::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crossroads {
    pub left: Node,
    pub right: Node
}

impl From<&str> for Crossroads {
    fn from(value: &str) -> Self {
        let (left, right) = value.strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .split_once(", ").unwrap();

        Crossroads {
            left: left.into(),
            right: right.into()
        }
    }
}

