use std::collections::BTreeMap;
use std::{io, fmt};
use advent_code_lib::for_each_line;
use std::fmt::{Display, Formatter};

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(after_n_cycles(ConwayCubes::from(filename)?, 6, puzzle1_cycle).to_string())
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

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            State::ACTIVE => '#',
            State::INACTIVE => '.'
        })
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct ConwayCubes {
    cubes: BTreeMap<Point3D,State>
}

impl ConwayCubes {
    pub fn from(filename: &str) -> io::Result<ConwayCubes> {
        let mut cubes = ConwayCubes { cubes: BTreeMap::new() };
        let mut y = 0;
        for_each_line(filename, |line| Ok({
            let mut x = 0;
            for c in line.chars() {
                cubes.cubes.insert(Point3D {x, y, z: 0}, State::from(c));
                x += 1;
            }
            y += 1;
        }))?;
        Ok(cubes)
    }

    pub fn state(&self, p: &Point3D) -> State {
        match self.cubes.get(&p) {
            None => State::INACTIVE,
            Some(s) => *s
        }
    }

    pub fn min_point(&self) -> Point3D {
        self.cubes.first_key_value().unwrap().0.clone()
    }

    pub fn max_point(&self) -> Point3D {
        self.cubes.last_key_value().unwrap().0.clone()
    }

    pub fn num_active_neighbors(&self, p: &Point3D) -> usize {
        p.neighbors().filter(|n| self.state(n) == State::ACTIVE).count()
    }

    pub fn num_active(&self) -> usize {
        self.cubes.values().filter(|v| **v == State::ACTIVE).count()
    }
}

impl Display for ConwayCubes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut prev = self.min_point().prev_corner();
        for (p, s) in self.cubes.iter() {
            if p.z != prev.z {write!(f, "\nz={}", p.z).unwrap();}
            if p.y != prev.y {writeln!(f, "").unwrap();}
            write!(f, "{}", s).unwrap();
            prev = p.clone();
        }
        writeln!(f, "")
    }
}

#[derive(Clone,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub struct Point3D {
    z: isize, y: isize, x: isize,
}

impl Point3D {
    pub fn prev_corner(&self) -> Point3D {
        Point3D {x: self.x - 1, y: self.y - 1, z: self.z - 1}
    }

    pub fn next_corner(&self) -> Point3D {
        Point3D {x: self.x + 1, y: self.y + 1, z: self.z + 1}
    }

    pub fn next(&self, start: &Point3D, end: &Point3D) -> Option<Point3D> {
        let mut next = self.clone();
        next.x += 1;
        if next.x > end.x {
            next.x = start.x;
            next.y += 1;
            if next.y > end.y {
                next.y = start.y;
                next.z += 1;
                if next.z > end.z {
                    return None;
                }
            }
        }
        Some(next)
    }

    pub fn neighbors(&self) -> impl Iterator<Item=Point3D> {
        let avoid = self.clone();
        Point3DIterator::new(&self.prev_corner(), &self.next_corner())
            .filter(move |n| n != &avoid)
    }
}

struct Point3DIterator {
    start: Point3D,
    end: Point3D,
    next: Option<Point3D>
}

impl Point3DIterator {
    pub fn new(start: &Point3D, end: &Point3D) -> Self {
        Point3DIterator { start: start.clone(), end: end.clone(), next: Some(start.clone())}
    }
}

impl Iterator for Point3DIterator {
    type Item = Point3D;

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

fn puzzle1_cycle(start: &ConwayCubes) -> ConwayCubes {
    let p_start = start.min_point().prev_corner();
    let p_end = start.max_point().next_corner();
    ConwayCubes {
        cubes: Point3DIterator::new(&p_start, &p_end)
            .map(|p| {
                let neighbor_active = start.num_active_neighbors(&p);
                (p.clone(), if neighbor_active == 3 || neighbor_active == 2 && start.state(&p) == State::ACTIVE {
                    State::ACTIVE
                } else {State::INACTIVE})
            })
            .collect()
    }
}

fn after_n_cycles<F:Fn(&ConwayCubes)->ConwayCubes>(start: ConwayCubes, n: usize, cycler: F) -> usize {
    let mut cubes = start;
    for _ in 0..n {
        cubes = cycler(&cubes);
    }
    cubes.num_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let points: Vec<Point3D> = Point3DIterator::new(&Point3D {x: -1, y: -1, z: -1}, &Point3D {x: 1, y: 1, z: 1}).collect();
        let target: Vec<Point3D> = [
            (-1, -1, -1), (0, -1, -1), (1, -1, -1),
            (-1,  0, -1), (0,  0, -1), (1,  0, -1),
            (-1,  1, -1), (0,  1, -1), (1,  1, -1),
            (-1, -1,  0), (0, -1,  0), (1, -1,  0),
            (-1,  0,  0), (0,  0,  0), (1,  0,  0),
            (-1,  1,  0), (0,  1,  0), (1,  1,  0),
            (-1, -1,  1), (0, -1,  1), (1, -1,  1),
            (-1,  0,  1), (0,  0,  1), (1,  0,  1),
            (-1,  1,  1), (0,  1,  1), (1,  1,  1),
        ].iter().map(|(x, y, z)| Point3D {x: *x, y: *y, z: *z}).collect();
        assert_eq!(points, target);
    }

    #[test]
    fn test_cubes() {
        let cubes = ConwayCubes::from("in/day17_ex.txt").unwrap();
        assert_eq!(format!("{}", cubes).as_str(), STEPS[0]);
    }

    #[test]
    fn test_puzzle_1() {
        let targets = [5, 11, 21, 38];
        let mut cubes = ConwayCubes::from("in/day17_ex.txt").unwrap();
        for t in 0..6 {
            if t < targets.len() {
                assert_eq!(cubes.num_active(), targets[t]);
            }
            cubes = puzzle1_cycle(&cubes);
        }
        assert_eq!(cubes.num_active(), 112);
    }

    const STEPS: [&str; 4] = [
        "
z=0
.#.
..#
###
",
        "
z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.
",
        "
z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....
",
        "
z=-2
.......
.......
..##...
..###..
.......
.......
.......

z=-1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=0
...#...
.......
#......
.......
.....##
.##.#..
...#...

z=1
..#....
...#...
#......
.....##
.#...#.
..#.#..
...#...

z=2
.......
.......
..##...
..###..
.......
.......
.......
"
    ];
}