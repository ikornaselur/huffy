use std::cmp::Ordering;
type ChildNode = Option<Box<Node>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub value: u8,
    pub weight: usize,
    pub left: ChildNode,
    pub right: ChildNode,
}

impl Node {
    pub fn is_leaf(self: &Node) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_leaf() {
        let node = Node {
            value: 3,
            weight: 7,
            left: None,
            right: None,
        };

        assert_eq!(node.value, 3);
        assert_eq!(node.weight, 7);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
        assert!(node.is_leaf());
    }

    #[test]
    fn test_node_with_children() {
        let mut parent = Node {
            value: 10,
            weight: 7,
            left: None,
            right: None,
        };

        parent.left = Some(Box::new(Node {
            value: 3,
            weight: 1,
            left: None,
            right: None,
        }));

        parent.right = Some(Box::new(Node {
            value: 9,
            weight: 2,
            left: None,
            right: None,
        }));

        assert_eq!(parent.value, 10);
        assert_eq!(parent.left.unwrap().value, 3);
        assert_eq!(parent.right.unwrap().value, 9);
    }
}
