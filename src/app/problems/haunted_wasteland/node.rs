#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node([char; 3]);

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let value = value.chars().collect::<Vec<char>>();
        Node([value[0], value[1], value[2]])
    }
}

impl Node {
    pub fn new(name: &str) -> Node {
        let chars = name.chars().collect::<Vec<char>>();
        Node([chars[0], chars[1], chars[2]])
    }

    pub fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }

    pub fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

