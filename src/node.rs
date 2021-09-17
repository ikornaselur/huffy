use std::cmp::Ordering;
type ChildNode = Option<Box<Node>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub value: Option<u8>,
    pub weight: usize,
    pub left: ChildNode,
    pub right: ChildNode,
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
            value: Some(3),
            weight: 7,
            left: None,
            right: None,
        };

        assert_eq!(node.value, Some(3));
        assert_eq!(node.weight, 7);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_node_with_children() {
        let mut parent = Node {
            value: None,
            weight: 7,
            left: None,
            right: None,
        };

        parent.left = Some(Box::new(Node {
            value: Some(3),
            weight: 1,
            left: None,
            right: None,
        }));

        parent.right = Some(Box::new(Node {
            value: Some(9),
            weight: 2,
            left: None,
            right: None,
        }));

        assert_eq!(parent.value, None);
        assert_eq!(parent.left.unwrap().value, Some(3));
        assert_eq!(parent.right.unwrap().value, Some(9));
    }
}
