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
    fn from_str(content: &str) -> Self {
        Node::Leaf(content.to_string())
    }

    fn get_wei(&self) -> usize {
        match self {
            Node::Leaf(content) => content.len(),
            Node::Internal { wei, .. } => *wei,
        }
    }
}

#[derive(Debug)]
struct Rope {
    wei: usize,
    root: Option<Box<Node>>,
}

impl Rope {
    fn from_str(arg: &str) -> Self {
        Self {
            wei: arg.len(),
            root: Some(Box::new(Node::Leaf(arg.to_string()))),
        }
    }

    fn concat(&mut self, other: Rope) {
        let wei = self.wei + other.wei;
        let n = Node::Internal {
            wei,
            left: self.root.take(),
            right: other.root,
        };

        self.root = Some(Box::new(n));
        self.wei = wei;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_rope() {
        let mut rop = Rope::from_str("i rope this works");
        let rop2 = Rope::from_str(", it will!!");

        rop.concat(rop2);

        assert_eq!(rop.wei, 28);
    }
}
