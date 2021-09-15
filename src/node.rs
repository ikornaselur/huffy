type ChildNode = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    value: u8,
    weight: f32,
    left: ChildNode,
    right: ChildNode,
}

impl Node {
    pub fn is_leaf(self: &Node) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_leaf() {
        let node = Node {
            value: 3,
            weight: 0.3,
            left: None,
            right: None,
        };

        assert_eq!(node.value, 3);
        assert!((node.weight - 0.3).abs() < f32::EPSILON);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
        assert!(node.is_leaf());
    }

    #[test]
    fn test_node_with_children() {
        let mut parent = Node {
            value: 10,
            weight: 0.3,
            left: None,
            right: None,
        };

        parent.left = Some(Box::new(Node {
            value: 3,
            weight: 0.1,
            left: None,
            right: None,
        }));

        parent.right = Some(Box::new(Node {
            value: 9,
            weight: 0.2,
            left: None,
            right: None,
        }));

        assert_eq!(parent.value, 10);
        assert_eq!(parent.left.unwrap().value, 3);
        assert_eq!(parent.right.unwrap().value, 9);
    }
}
