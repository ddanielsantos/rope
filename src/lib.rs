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

#[derive(Debug, Default, PartialEq)]
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

            match *root {
                Node::Leaf(ref mut content) => {
                    let ri = content.split_off(pos);

                    self.root = Some(root);

                    Rope {
                        wei: ri.len(),
                        root: Some(Box::new(Node::Leaf(ri))),
                    }
                }
                Node::Internal { wei, left, right } => todo!(),
            }
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

    #[test]
    fn split_rope2() {
        let mut rop = Rope {
            wei: 29,
            root: Some(Box::new(Node::Internal {
                wei: 29,
                left: Some(Box::new(Node::Internal {
                    wei: 10,
                    left: Some(Box::new(Node::Internal {
                        wei: 3,
                        left: Some(Box::new(Node::from_str("Ola"))),
                        right: Some(Box::new(Node::from_str(" mundo."))),
                    })),
                    right: Some(Box::new(Node::Internal {
                        wei: 5,
                        left: Some(Box::new(Node::from_str(" Meu "))),
                        right: Some(Box::new(Node::from_str("nome "))),
                    })),
                })),
                right: Some(Box::new(Node::Internal {
                    wei: 9,
                    left: Some(Box::new(Node::Internal {
                        wei: 3,
                        left: Some(Box::new(Node::from_str("eh "))),
                        right: Some(Box::new(Node::from_str("Daniel"))),
                    })),
                    right: None,
                })),
            })),
        };

        let remainder = rop.split(13);

        let expected = Rope {
            wei: 10,
            root: Some(Box::new(Node::Internal {
                wei: 3,
                left: Some(Box::new(Node::from_str("Ola"))),
                right: Some(Box::new(Node::from_str(" mundo."))),
            })),
        };

        assert_eq!(rop.wei, 13);
        assert_eq!(rop, expected);
        assert_eq!(remainder.wei, 18);
    }
}
