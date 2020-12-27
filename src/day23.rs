use num::Integer;
use std::collections::BTreeSet;

pub fn solve_1(cups: [u8; 9]) -> String {
    let mut cups = CupRing::new(cups, cups.len());
    for _ in 0..100 {
        cups.move_once()
    }
    cups.num_string()
}

pub fn solve_2(cups: [u8; 9]) -> String {
    let mut cups = CupRing::new(cups, 1_000_000);
    for _ in 0..10_000_000 {
        cups.move_once();
    }
    cups.star_product().to_string()
}

#[derive(Debug,Clone,Copy)]
struct CupNode {
    value: u32,
    next: usize
}

struct CupNodeIter<'a> {
    next: usize,
    ring: &'a CupRing
}

impl <'a> Iterator for CupNodeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let was = self.next;
        self.next = self.ring.cups[was].next;
        Some(was)
    }
}

#[derive(Debug,Clone)]
struct CupRing {
    cups: Vec<CupNode>,
    values2pointers: Vec<usize>,
    current: usize,
    min: u32,
    max: u32
}

const NUM_REMOVE: usize = 3;

impl CupRing {
    fn new(ordering: [u8; 9], total: usize) -> Self {
        let cups: Vec<CupNode> = ordering.iter().map(|v| *v as u32)
            .chain((ordering.len()..total).map(|i| (i+1) as u32))
            .enumerate()
            .map(|(i, value)| CupNode { value, next: (i + 1).mod_floor(&total) })
            .collect();
        let mut values2pointers: Vec<usize> = std::iter::repeat(0).take(total + 1).collect();
        for (i, node) in cups.iter().enumerate() {
            values2pointers[node.value as usize] = i;
        }
        CupRing { cups, values2pointers, current: 0, min: 1, max: total as u32 }
    }

    fn move_once(&mut self) {
        let (dest_ptr, remove_end_ptr) = self.destination_remove_end_ptrs();
        let remove_start_ptr = self.cups[self.current].next;
        let after_dest_ptr = self.cups[dest_ptr].next;
        self.cups[self.current].next = self.cups[remove_end_ptr].next;
        self.cups[dest_ptr].next = remove_start_ptr;
        self.cups[remove_end_ptr].next = after_dest_ptr;
        self.current = self.cups[self.current].next;
    }

    fn destination_remove_end_ptrs(&self) -> (usize, usize) {
        let mut remove_end_ptr = 0;
        let mut destination_value = self.destination_label_sub(self.cups[self.current].value);
        let mut destination_finder = self.iter().skip(1);
        let mut values = BTreeSet::new();
        destination_finder.by_ref()
            .take(NUM_REMOVE)
            .for_each(|p| {
                remove_end_ptr = p;
                values.insert(self.cups[p].value);
            });
        while values.contains(&destination_value) {
            destination_value = self.destination_label_sub(destination_value);
        }
        (self.values2pointers[destination_value as usize], remove_end_ptr)
    }

    fn destination_label_sub(&self, label: u32) -> u32 {
        let mut label = label - 1;
        if label < self.min {
            label = self.max;
        }
        label
    }

    fn iter_after_1(&self) -> CupNodeIter {
        let mut iter = self.iter();
        iter.by_ref().skip_while(|p| self.cups[*p].value != 1).next();
        iter
    }

    fn num_string(&self) -> String {
        self.iter_after_1()
            .take_while(|p| self.cups[*p].value != 1)
            .map(|p| (self.cups[p].value as u8 + '0' as u8) as char)
            .collect()
    }

    fn star_product(&self) -> u64 {
        self.iter_after_1()
            .take(2)
            .map(|p| self.cups[p].value as u64)
            .product()
    }

    fn iter(&self) -> CupNodeIter {
        CupNodeIter {next: self.current, ring: &self}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn puzzle_1_example() -> CupRing {
        CupRing::new([3, 8, 9, 1, 2, 5, 4, 6, 7], 9)
    }

    #[test]
    fn test_moves() {
        let mut cups = puzzle_1_example();
        for target in [
            "54673289",
            "32546789",
            "34672589",
            "32584679",
            "36792584",
            "93672584",
            "92583674",
            "58392674",
            "83926574",
            "92658374"
        ].iter() {
            cups.move_once();
            assert_eq!(&cups.num_string(), target);
        }
        assert_eq!(cups.num_string(), "92658374");
    }

    #[test]
    fn bigger_test() {
        assert_eq!(solve_1([3, 8, 9, 1, 2, 5, 4, 6, 7]), "67384529");
    }

    fn assert_no_nodes_lost(ring: &CupRing) {
        let ptr_set: BTreeSet<usize> = ring.cups.iter().map(|p| p.next).collect();
        assert_eq!(ptr_set.len(), ring.cups.len());
    }

    #[test]
    fn slightly_big_test() {
        let mut cups = CupRing::new([3, 8, 9, 1, 2, 5, 4, 6, 7], 1_000_000);
        for _ in 0..20 {
            cups.move_once();
            assert_no_nodes_lost(&cups);
        }
    }

    #[test]
    fn huge_test() {
        assert_eq!(solve_2([3, 8, 9, 1, 2, 5, 4, 6, 7]), "149245887792");
    }
}