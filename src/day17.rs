use std::collections::BTreeMap;
use std::io;
use advent_code_lib::for_each_line;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(after_n_cycles(ConwayCubes::from(filename, 3)?, 6).to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(after_n_cycles(ConwayCubes::from(filename, 4)?, 6).to_string())
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum State {
    ACTIVE, INACTIVE
}

impl State {
    pub fn from(c: char) -> State {
        match c {
            '#' => State::ACTIVE,
            '.' => State::INACTIVE,
            _ => panic!("Unrecognized input character: '{}'", c)
        }
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct ConwayCubes {
    cubes: BTreeMap<PointND,State>
}

impl ConwayCubes {
    pub fn from(filename: &str, dimension: usize) -> io::Result<ConwayCubes> {
        let mut cubes = ConwayCubes { cubes: BTreeMap::new() };
        let mut y = 0;
        for_each_line(filename, |line| Ok({
            let mut x = 0;
            for c in line.chars() {
                cubes.cubes.insert(PointND::new_zero_pad(&[x, y], dimension), State::from(c));
                x += 1;
            }
            y += 1;
        }))?;
        Ok(cubes)
    }

    pub fn state(&self, p: &PointND) -> State {
        match self.cubes.get(&p) {
            None => State::INACTIVE,
            Some(s) => *s
        }
    }

    pub fn min_point(&self) -> PointND {
        self.cubes.first_key_value().unwrap().0.clone()
    }

    pub fn max_point(&self) -> PointND {
        self.cubes.last_key_value().unwrap().0.clone()
    }

    pub fn num_active_neighbors(&self, p: &PointND) -> usize {
        p.neighbors().filter(|n| self.state(n) == State::ACTIVE).count()
    }

    pub fn num_active(&self) -> usize {
        self.cubes.values().filter(|v| **v == State::ACTIVE).count()
    }
}

#[derive(Clone,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub struct PointND {
    coords: Vec<isize>
}

impl PointND {
    pub fn new(coords: &[isize]) -> PointND {
        PointND { coords: Vec::from(coords) }
    }

    pub fn new_zero_pad(coords: &[isize], target_len: usize) -> PointND {
        let mut result = PointND::new(coords);
        for _ in result.coords.len()..target_len {
            result.coords.push(0);
        }
        result
    }

    pub fn prev_corner(&self) -> PointND {
        PointND { coords: self.coords.iter().map(|c| c - 1).collect()}
    }

    pub fn next_corner(&self) -> PointND {
        PointND { coords: self.coords.iter().map(|c| c + 1).collect()}
    }

    pub fn next(&self, start: &PointND, end: &PointND) -> Option<PointND> {
        let mut next = self.clone();
        let mut c = 0;
        while c < next.coords.len() {
            next.coords[c] += 1;
            if next.coords[c] > end.coords[c] {
                next.coords[c] = start.coords[c];
                c += 1;
            } else {
                return Some(next);
            }
        }
        None
    }

    pub fn neighbors(&self) -> impl Iterator<Item=PointND> {
        let avoid = self.clone();
        PointNDIterator::new(&self.prev_corner(), &self.next_corner())
            .filter(move |n| n != &avoid)
    }
}

struct PointNDIterator {
    start: PointND,
    end: PointND,
    next: Option<PointND>
}

impl PointNDIterator {
    pub fn new(start: &PointND, end: &PointND) -> Self {
        PointNDIterator { start: start.clone(), end: end.clone(), next: Some(start.clone())}
    }
}

impl Iterator for PointNDIterator {
    type Item = PointND;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == None {
            None
        } else {
            let result = self.next.clone();
            self.next = self.next.clone().unwrap().next(&self.start, &self.end);
            result
        }
    }
}

fn new_cell_state(cell: State, num_active_neighbors: usize) -> State {
    if num_active_neighbors == 3 || num_active_neighbors == 2 && cell == State::ACTIVE {
        State::ACTIVE
    } else {
        State::INACTIVE
    }
}

fn cycle(start: &ConwayCubes) -> ConwayCubes {
    ConwayCubes {
        cubes: PointNDIterator::new(&start.min_point().prev_corner(), &start.max_point().next_corner())
            .map(|p| (p.clone(), new_cell_state(start.state(&p),
                                                start.num_active_neighbors(&p))))
            .collect()
    }
}

fn after_n_cycles(start: ConwayCubes, n: usize) -> usize {
    let mut cubes = start;
    for _ in 0..n {
        cubes = cycle(&cubes);
    }
    cubes.num_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let points: Vec<PointND> = PointNDIterator::new(&PointND::new(&[-1, -1, -1]), &PointND::new(&[1, 1, 1])).collect();
        let target: Vec<PointND> = [
            (-1, -1, -1), (0, -1, -1), (1, -1, -1),
            (-1,  0, -1), (0,  0, -1), (1,  0, -1),
            (-1,  1, -1), (0,  1, -1), (1,  1, -1),
            (-1, -1,  0), (0, -1,  0), (1, -1,  0),
            (-1,  0,  0), (0,  0,  0), (1,  0,  0),
            (-1,  1,  0), (0,  1,  0), (1,  1,  0),
            (-1, -1,  1), (0, -1,  1), (1, -1,  1),
            (-1,  0,  1), (0,  0,  1), (1,  0,  1),
            (-1,  1,  1), (0,  1,  1), (1,  1,  1),
        ].iter().map(|(x, y, z)| PointND::new(&[*x, *y, *z])).collect();
        assert_eq!(points, target);
    }

    #[test]
    fn test_puzzle_1() {
        let targets = [5, 11, 21, 38];
        let mut cubes = ConwayCubes::from("in/day17_ex.txt", 3).unwrap();
        for t in 0..6 {
            if t < targets.len() {
                assert_eq!(cubes.num_active(), targets[t]);
            }
            cubes = cycle(&cubes);
        }
        assert_eq!(cubes.num_active(), 112);
    }

    #[test]
    #[ignore] // It works, but it takes about 50 seconds on my laptop
    fn test_puzzle_2() {
        let cubes = ConwayCubes::from("in/day17_ex.txt", 4).unwrap();
        assert_eq!(after_n_cycles(cubes, 6), 848);
    }
}