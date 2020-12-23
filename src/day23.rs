use num::Integer;

pub fn solve_1(cups: [u8; 9]) -> String {
    let mut cups = CupRing::new(cups);
    for _ in 0..100 {
        cups.move_once()
    }
    cups.num_string()
}

#[derive(Debug,Clone)]
struct CupRing {
    cups: [u8; 9],
    current: usize
}

const NUM_REMOVE: usize = 3;

impl CupRing {
    fn new(ordering: [u8; 9]) -> Self {
        CupRing {cups: ordering, current: 0}
    }

    fn ind_add(&self, a: usize, b: usize) -> usize {
        (a + b).mod_floor(&self.cups.len())
    }

    fn ind_sub(&self, a: usize, b: usize) -> usize {
        self.ind_add(a, self.cups.len() - b)
    }

    fn ind_iter(&self, start: usize, end: usize) -> IndIter {
        IndIter::from(start, end, self.cups.len())
    }

    fn move_once(&mut self) {
        let remove_start = self.ind_add(self.current, 1);
        let retain_start = self.ind_add(remove_start, NUM_REMOVE);
        let destination = self.find_destination_index(retain_start);
        self.remove_insert(remove_start, 3, self.ind_sub(destination + 1, NUM_REMOVE));
        self.current = self.ind_add(self.current, 1);
    }

    fn destination_label_sub(&self, label: u8) -> u8 {
        let mut label = label - 1;
        if label < *self.cups.iter().min().unwrap() {
            label = *self.cups.iter().max().unwrap();
        }
        label
    }

    fn find_destination_index(&self, retain_start: usize) -> usize {
        let mut destination_label = self.destination_label_sub(self.cups[self.current]);
        loop {
            match self.ind_iter(retain_start, self.current)
                .find(|i| self.cups[*i] == destination_label) {
                Some(ind) => return ind,
                None => destination_label = self.destination_label_sub(destination_label)
            }
        }
    }

    fn remove_insert(&mut self, remove_start: usize, num_remove: usize, insert_start: usize) {
        let holding: Vec<u8> = self.ind_iter(remove_start, remove_start + num_remove)
            .map(|i| self.cups[i])
            .collect();
        for leftward in self.ind_iter(remove_start, insert_start) {
            self.cups[leftward] = self.cups[self.ind_add(leftward, num_remove)];
        }
        for (i, v) in holding.iter().enumerate() {
            self.cups[self.ind_add(insert_start, i)] = *v;
        }
    }

    fn num_string(&self) -> String {
        let one_at = self.cups.iter().enumerate().find(|(_,c)| **c == 1).map(|(i,_)| i).unwrap();
        self.ind_iter(self.ind_add(one_at, 1), one_at).map(|i| (self.cups[i] + '0' as u8) as char).collect()
    }
}

struct IndIter {
    next: Option<usize>, done: usize, max: usize
}

impl IndIter {
    fn from(start: usize, open_end: usize, max: usize) -> Self {
        IndIter { next: Some(start.mod_floor(&max)), done: open_end.mod_floor(&max), max}
    }
}

impl Iterator for IndIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;
        if let Some(n) = result {
            let update = (n + 1).mod_floor(&self.max);
            self.next = if update == self.done {None} else {Some(update)};
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ind_iter() {
        [
            (4, 8, 9, vec![4, 5, 6, 7]),
            (5, 0, 9, vec![5, 6, 7, 8]),
            (6, 1, 9, vec![6, 7, 8, 0])
        ].iter().for_each(|(start, end, max, target)| {
            assert_eq!(&IndIter::from(*start, *end, *max).collect::<Vec<usize>>(), target);
        });
    }

    #[test]
    fn test_remove_insert() {
        let mut cups = CupRing::new([3, 8, 9, 1, 2, 5, 4, 6, 7]);
        cups.remove_insert(1, 3, 2);
        assert_eq!(cups.cups, [3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }

    #[test]
    fn test_find_destination() {
        let cups1 = CupRing::new([3, 8, 9, 1, 2, 5, 4, 6, 7]);
        assert_eq!(cups1.find_destination_index(4), 4);
        let cups2 = CupRing::new([3, 2, 8, 9, 1, 5, 4, 6, 7]);
        assert_eq!(cups2.find_destination_index(5), 8);
    }

    #[test]
    fn test_moves() {
        let mut cups = CupRing::new([3, 8, 9, 1, 2, 5, 4, 6, 7]);
        for target in [
            [3, 2, 8, 9, 1, 5, 4, 6, 7],
            [3, 2, 5, 4, 6, 7, 8, 9, 1],
            [7, 2, 5, 8, 9, 1, 3, 4, 6],
            [3, 2, 5, 8, 4, 6, 7, 9, 1],
            [9, 2, 5, 8, 4, 1, 3, 6, 7],
            [7, 2, 5, 8, 4, 1, 9, 3, 6],
            [8, 3, 6, 7, 4, 1, 9, 2, 5],
            [7, 4, 1, 5, 8, 3, 9, 2, 6],
            [5, 7, 4, 1, 8, 3, 9, 2, 6],
            [5, 8, 3, 7, 4, 1, 9, 2, 6]
        ].iter() {
            cups.move_once();
            assert_eq!(&cups.cups, target);
        }
        assert_eq!(cups.num_string(), "92658374");
    }

    #[test]
    fn bigger_test() {
        assert_eq!(solve_1([3, 8, 9, 1, 2, 5, 4, 6, 7]), "67384529");
    }
}