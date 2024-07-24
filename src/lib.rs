#[derive(Debug, PartialEq)]
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

    fn split(&mut self, pos: usize) -> Rope {
        if self.wei == 0 {
            return Rope::default();
        }

        self.root.take().map_or_else(Rope::default, |mut root| {
            self.wei = pos;

            let rope = match *root {
                Node::Leaf(ref mut content) => {
                    let ri = content.split_off(pos);

                    self.root = Some(root);

                    Rope {
                        wei: ri.len(),
                        root: Some(Box::new(Node::Leaf(ri))),
                    }
                }
                Node::Internal { wei, left, right } => todo!(),
            };

            rope
        })
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
        let mut rop = Rope::from_str("Daniel");
        let remainder = rop.split(4);

        assert_eq!(rop.wei, 4);
        assert_eq!(rop.root, Some(Box::new(Node::Leaf("Dani".to_string()))));
        assert_eq!(remainder.wei, 2);
    }
}
