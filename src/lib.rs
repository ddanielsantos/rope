use std::ops::Range;

#[derive(Debug)]
enum Node {
    Leaf(String),
    Internal {
        wei: usize,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
}

impl Node {
    fn new() -> Self {
        Node::Leaf("".to_string())
    }
}

#[derive(Debug)]
struct Rope {
    wei: usize,
    root: Option<Node>,
}

impl Rope {
    fn new() -> Self {
        Self { wei: 0, root: None }
    }

    fn push(&mut self, input: &str) -> () {
        todo!()
    }

    fn append(&self, other: Rope) -> () {
        todo!()
    }

    fn replace(&mut self, range: Range<i32>, input: &str) -> () {
        todo!()
    }

    fn text(&self) -> String {
        let a = &self.root;

        match a {
            Some(node) => match node {
                Node::Leaf(content) => content.to_string(),
                Node::Internal { wei, left, right } => todo!(),
            },
            None => todo!(),
        }
    }

    fn from_str(arg: &str) -> Self {
        Self {
            wei: arg.len(),
            root: Some(Node::Leaf(arg.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rope = Rope::from_str("One coffee, please. Black, yes.");

        // Replace characters 4 to 10 (0-indexed) with "guinness".
        rope.replace(4..10, "guinness");
        assert_eq!(rope.text(), "One guinness, please. Black, yes.");
    }
}
