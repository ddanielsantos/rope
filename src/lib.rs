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

#[derive(Debug, Default)]
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

    fn split(&mut self, arg: usize) -> Rope {
        if self.wei == 0 {
            return Rope::default();
        }

        Rope::from_str("idk yet")
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

    #[test]
    fn split_rope() {
        let mut rop = Rope::from_str("Hello,");
        rop.concat(Rope::from_str(" world!"));

        println!("{rop:?}");
        assert_eq!(rop.wei, 13);

        let remainder = rop.split(10);

        println!("{rop:?}");
        assert_eq!(rop.wei, 10);
        assert_eq!(remainder.wei, 3);
    }
}
