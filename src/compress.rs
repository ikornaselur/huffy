use crate::node::Node;
use anyhow::Result;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

const BUFFER_SIZE: usize = 512;

pub fn compress(file: File) -> Result<()> {
    // Count the occurrances of bytes
    let buf = BufReader::new(file);
    let counts = get_counts(buf)?;
    let heap = counts_to_heap(counts);
    if let Some(head) = heap_to_tree(heap) {
        println!("Got head with val {}!", head.value);
    } else {
        println!("No head!");
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
                value: idx as u8,
                weight: *count,
                left: None,
                right: None,
            }));
        }
    }

    heap
}

fn heap_to_tree(heap: BinaryHeap<Reverse<Node>>) -> Option<Node> {
    None
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
        assert_eq!(node.value, 0);
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
        assert_eq!(node.value, 1);
        assert_eq!(node.weight, 3);

        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, 0);
        assert_eq!(node.weight, 10);

        let Reverse(node) = heap.pop().unwrap();
        assert_eq!(node.value, 2);
        assert_eq!(node.weight, 25);
    }
}
