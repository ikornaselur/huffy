use crate::node::Node;
use anyhow::Result;
use bit_vec::BitVec;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

const BUFFER_SIZE: usize = 512;

pub fn compress(file: File) -> Result<()> {
    let buf = BufReader::new(file);
    let counts = get_counts(buf)?;
    let heap = counts_to_heap(counts);
    if let Some(head) = heap_to_tree(heap) {
        let _bitmap = tree_to_bit_hash_map(head);
        for (key, value) in _bitmap.iter() {
            println!("'{}': {:?}", *key as char, value);
        }
    } else {
        return Ok(());
    }

    Ok(())
}

fn get_counts<T: Read>(mut buffer: BufReader<T>) -> Result<[usize; 256]> {
    let mut counts = [0; 256];
    let mut bytes = [0; BUFFER_SIZE];

    loop {
        match buffer.read(&mut bytes) {
            Ok(0) => break,
            Ok(n) => {
                for i in 0..n {
                    counts[bytes[i] as usize] += 1;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!("{:?}", e),
        };
    }

    Ok(counts)
}

fn counts_to_heap(counts: [usize; 256]) -> BinaryHeap<Reverse<Node>> {
    let mut heap = BinaryHeap::new();

    for (idx, count) in counts.iter().enumerate() {
        if count > &0 {
            heap.push(Reverse(Node {
                value: Some(idx as u8),
                weight: *count,
                left: None,
                right: None,
            }));
        }
    }

    heap
}

fn heap_to_tree(mut heap: BinaryHeap<Reverse<Node>>) -> Option<Node> {
    while heap.len() > 1 {
        let Reverse(a) = heap.pop().unwrap();
        let Reverse(b) = heap.pop().unwrap();
        let node = Node {
            value: None,
            weight: a.weight + b.weight,
            left: Some(Box::new(a)),
            right: Some(Box::new(b)),
        };
        heap.push(Reverse(node));
    }
    if let Some(Reverse(head)) = heap.pop() {
        Some(head)
    } else {
        None
    }
}

fn tree_to_bit_hash_map(head: Node) -> HashMap<u8, BitVec> {
    let mut queue = VecDeque::new();
    let mut map = HashMap::new();
    let bit_vec = BitVec::new();

    queue.push_back((head, bit_vec));

    while !queue.is_empty() {
        let (node, bits) = queue.pop_front().unwrap();
        if let Some(value) = node.value {
            map.insert(value, bits);
        } else {
            if let Some(left) = node.left {
                let mut left_bits = bits.clone();
                left_bits.push(false);
                queue.push_back((*left, left_bits));
            }
            if let Some(right) = node.right {
                let mut right_bits = bits.clone();
                right_bits.push(true);
                queue.push_back((*right, right_bits));
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_counts_no_bytes() {
        let file = Vec::new();
        let cursor = Cursor::new(file);
        let buffer = BufReader::new(cursor);

        let counts = get_counts(buffer).unwrap();

        let expected = [0; 256];

        assert_eq!(counts, expected);
    }

    #[test]
    fn test_get_counts_few_bytes() {
        let file = Vec::from([0, 255, 0, 5, 255, 0]);
        let cursor = Cursor::new(file);
        let buffer = BufReader::new(cursor);

        let counts = get_counts(buffer).unwrap();

        let mut expected = [0; 256];
        expected[0] = 3;
        expected[255] = 2;
        expected[5] = 1;

        assert_eq!(counts, expected);
    }

    #[test]
    fn test_counts_to_queue_empty_queue() {
        let counts = [0; 256];

        let heap = counts_to_heap(counts);

        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_counts_to_single_value() {
        let mut counts = [0; 256];
        counts[0] = 10;

        let mut heap = counts_to_heap(counts);

        assert_eq!(heap.len(), 1);
        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, Some(0));
        assert_eq!(node.weight, 10);
    }

    #[test]
    fn test_counts_to_multiple_values_has_lowest_weight_first() {
        let mut counts = [0; 256];
        counts[0] = 10;
        counts[1] = 3;
        counts[2] = 25;

        let mut heap = counts_to_heap(counts);

        assert_eq!(heap.len(), 3);

        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, Some(1));
        assert_eq!(node.weight, 3);

        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, Some(0));
        assert_eq!(node.weight, 10);

        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, Some(2));
        assert_eq!(node.weight, 25);
    }

    #[test]
    fn test_heap_to_tree_with_no_values() {
        let heap = BinaryHeap::new();

        let head = heap_to_tree(heap);

        assert!(head.is_none());
    }

    #[test]
    fn test_heap_to_tree_with_one_node() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Node {
            value: Some(5),
            weight: 10,
            left: None,
            right: None,
        }));

        let head = heap_to_tree(heap);

        assert_eq!(head.unwrap().value, Some(5));
    }

    #[test]
    fn test_heap_to_tree_with_with_multiple_values() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(Node {
            value: Some(5),
            weight: 10,
            left: None,
            right: None,
        }));
        heap.push(Reverse(Node {
            value: Some(1),
            weight: 3,
            left: None,
            right: None,
        }));
        heap.push(Reverse(Node {
            value: Some(2),
            weight: 6,
            left: None,
            right: None,
        }));
        heap.push(Reverse(Node {
            value: Some(9),
            weight: 8,
            left: None,
            right: None,
        }));

        /* Expected tree:
         *
         *     *:27
         *   /      \
         * 5:10    *:17
         *       /      \
         *     9:8      *:9
         *             /   \
         *           1:3   2:6
         */
        let head = heap_to_tree(heap).unwrap();
        assert_eq!(head.value, None);
        assert_eq!(head.weight, 27);

        let head_left = head.left.unwrap();
        assert_eq!(head_left.value, Some(5));
        assert_eq!(head_left.weight, 10);

        let head_right = head.right.unwrap();
        assert_eq!(head_right.value, None);
        assert_eq!(head_right.weight, 17);

        let head_right_left = head_right.left.unwrap();
        assert_eq!(head_right_left.value, Some(9));
        assert_eq!(head_right_left.weight, 8);

        let head_right_right = head_right.right.unwrap();
        assert_eq!(head_right_right.value, None);
        assert_eq!(head_right_right.weight, 9);

        let head_right_right_left = head_right_right.left.unwrap();
        assert_eq!(head_right_right_left.value, Some(1));
        assert_eq!(head_right_right_left.weight, 3);

        let head_right_right_right = head_right_right.right.unwrap();
        assert_eq!(head_right_right_right.value, Some(2));
        assert_eq!(head_right_right_right.weight, 6);
    }

    #[test]
    fn test_tree_to_bit_hash_map() {
        let head = Node {
            value: None,
            weight: 27,
            left: Some(Box::new(Node {
                value: Some(3),
                weight: 17,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: Some(8),
                weight: 10,
                left: None,
                right: None,
            })),
        };

        let bitmap = tree_to_bit_hash_map(head);

        assert!(bitmap.get(&3u8).unwrap().eq_vec(&[false]));
        assert!(bitmap.get(&8u8).unwrap().eq_vec(&[true]));
    }
}
