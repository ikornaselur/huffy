use bit_vec::BitVec;
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

impl Node {
    /// Export the node and all it's children into a bitvector that can be imported later. The
    /// weight is excluded from the bitvector, as it's never needed after being imported.
    /// Importing is only done when decoding the huffman coded file.
    pub fn export(&self) -> BitVec {
        match self.value {
            Some(val) => {
                let mut bits = BitVec::from_elem(1, true);
                bits.extend(BitVec::from_bytes(&[val]));
                bits
            }
            _ => {
                let mut bits = BitVec::from_elem(1, false);
                if let Some(left) = &self.left {
                    bits.extend(left.export());
                }
                if let Some(right) = &self.right {
                    bits.extend(right.export());
                }
                bits
            }
        }
    }

    pub fn import(&self, bits: BitVec) -> Self {
        Node {
            value: None,
            weight: 0,
            left: None,
            right: None,
        }
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

    #[test]
    fn test_node_export_leaf() {
        let node = Node {
            value: Some(3),
            weight: 3,
            left: None,
            right: None,
        };

        let bits = node.export();

        // 1 followed by 0b00000011
        assert!(bits.eq_vec(&[true, false, false, false, false, false, false, true, true]));
    }

    #[test]
    fn test_node_export_node_with_children() {
        let mut node = Node {
            value: None,
            weight: 5,
            left: None,
            right: None,
        };
        node.left = Some(Box::new(Node {
            value: Some(3),
            weight: 3,
            left: None,
            right: None,
        }));
        node.right = Some(Box::new(Node {
            value: Some(2),
            weight: 2,
            left: None,
            right: None,
        }));

        let bits = node.export();

        // 0 followed by two children:
        // 1 followed by 0b00000011
        // 1 followed by 0b00000010
        assert!(bits.eq_vec(&[
            false, true, false, false, false, false, false, false, true, true, true, false, false,
            false, false, false, false, true, false,
        ]));
    }
}
