use std::ops::{Add, AddAssign};
use std::collections::BTreeMap;
use std::{io, fmt};
use advent_code_lib::for_each_line;
use std::fmt::{Display, Formatter};

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

    pub fn state(&self, p: Point3D) -> State {
        match self.cubes.get(&p) {
            None => State::INACTIVE,
            Some(s) => *s
        }
    }
}

impl Display for ConwayCubes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut prev = *self.cubes.first_key_value().unwrap().0;
        let offset = Point3D {x: prev.x, y: prev.y - 1, z: prev.z - 1};
        prev += offset;
        for (p, s) in self.cubes.iter() {
            if p.z != prev.z {write!(f, "\nz={}", p.z).unwrap();}
            if p.y != prev.y {writeln!(f, "").unwrap();}
            write!(f, "{}", s).unwrap();
            prev = *p;
        }
        writeln!(f, "")
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub struct Point3D {
    z: isize, y: isize, x: isize,
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

struct Point3DIterator {
    start: Point3D,
    end: Point3D,
    next: Option<Point3D>
}

impl Point3DIterator {
    pub fn new(start: Point3D, end: Point3D) -> Self {
        Point3DIterator { start, end, next: Some(start)}
    }

    pub fn update(&mut self) {
        if let Some(mut next) = self.next {
            next.x += 1;
            if next.x > self.end.x {
                next.x = self.start.x;
                next.y += 1;
                if next.y > self.end.y {
                    next.y = self.start.y;
                    next.z += 1;
                    if next.z > self.end.z {
                        self.next = None;
                        return;
                    }
                }
            }
            self.next = Some(next);
        }
    }
}

impl Iterator for Point3DIterator {
    type Item = Point3D;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;
        self.update();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let points: Vec<Point3D> = Point3DIterator::new(Point3D {x: -1, y: -1, z: -1}, Point3D {x: 1, y: 1, z: 1}).collect();
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