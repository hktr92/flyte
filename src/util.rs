use std::convert::Into;

#[derive(Clone, Debug)]
pub struct Node(pub String);

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
