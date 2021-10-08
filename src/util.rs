use std::convert::Into;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Node(pub String);

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Node {
    pub fn new(key: String) -> Self {
        Self(key)
    }
}

impl Into<String> for Node {
    fn into(self) -> String {
        self.0
    }
}
