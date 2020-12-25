use advent_code_lib::{Position, all_lines};
use std::collections::BTreeMap;
use std::{io, mem};
use enum_iterator::IntoEnumIterator;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Floor::from(filename)?.count_color(TileColor::Black).to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let floor = Floor::from(filename)?;
    let after100 = FloorOfLifeIter {next: Some(floor)}.skip(100).next().unwrap();
    Ok(after100.count_color(TileColor::Black).to_string())
}

#[derive(Debug,Eq,PartialEq,Clone)]
struct Floor {
    floor: BTreeMap<Position,TileColor>
}

impl Floor {
    fn from(filename: &str) -> io::Result<Self> {
        let mut result = Floor { floor: BTreeMap::new() };
        all_lines(filename)?.for_each(|line| {
            let dirs = dir_seq(line.as_str());
            let destination = dirs.iter().fold(Position::new(), |p, hd| hd.next(p));
            match result.floor.get_mut(&destination) {
                None => {result.floor.insert(destination, TileColor::Black);}
                Some(color) => {color.flip();}
            }
        });
        Ok(result)
    }

    fn count_color(&self, color: TileColor) -> usize {
        self.floor.values().filter(|v| **v == color).count()
    }

    fn color(&self, p: Position) -> TileColor {
        match self.floor.get(&p) {
            None => TileColor::White,
            Some(c) => *c
        }
    }

    fn num_black_adj(&self, p: Position) -> usize {
        HexDir::into_enum_iter()
            .map(|dir| self.color(dir.next(p)))
            .filter(|c| *c == TileColor::Black)
            .count()
    }

    fn add_white_neighbors(&mut self) {
        let mut insertions = Vec::new();
        for (p, c) in self.floor.iter() {
            if *c == TileColor::Black {
                for n in HexDir::neighbors(*p) {
                    if !self.floor.contains_key(&n) {
                        insertions.push(n);
                    }
                }
            }
        }
        for insertion in insertions {
            self.floor.insert(insertion, TileColor::White);
        }
    }
}

struct FloorOfLifeIter {
    next: Option<Floor>
}

impl Iterator for FloorOfLifeIter {
    type Item = Floor;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &self.next {
            let mut future = current.clone();
            future.add_white_neighbors();
            for (p, c) in future.floor.iter_mut() {
                let black_adj = current.num_black_adj(*p);
                if *c == TileColor::White && black_adj == 2 || *c == TileColor::Black && (black_adj == 0 || black_adj > 2) {
                    c.flip();
                }
            }
            let mut result = Some(future);
            mem::swap(&mut result, &mut self.next);
            result
        } else {
            None
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum TileColor {
    White, Black
}

impl TileColor {
    fn flip(&mut self) {
        *self = match self {TileColor::Black => TileColor::White, TileColor::White => TileColor::Black}
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,IntoEnumIterator)]
enum HexDir {
    E, Se, Sw, W, Nw, Ne
}

impl HexDir {
    fn from(first: Option<char>, second: char) -> HexDir {
        match first {
            Some(c) => match c {
                'n' => match second {
                    'e' => HexDir::Ne,
                    'w' => HexDir::Nw,
                    _ => panic!("Unrecognized char: '{}'", second)
                }
                's' => match second {
                    'e' => HexDir::Se,
                    'w' => HexDir::Sw,
                    _ => panic!("Unrecognized char: '{}'", second)
                }
                _ => panic!("Unrecognized char: '{}", c)
            }
            None => match second {
                'e' => HexDir::E,
                'w' => HexDir::W,
                _ => panic!("Unrecognized char: '{}'", second)
            }
        }
    }

    fn next(&self, p: Position) -> Position {
        Position::from(
            match self {
                HexDir::E => (p.col + 2, p.row),
                HexDir::Ne => (p.col + 1, p.row - 1),
                HexDir::Se => (p.col + 1, p.row + 1),
                HexDir::W => (p.col - 2, p.row),
                HexDir::Nw => (p.col - 1, p.row - 1),
                HexDir::Sw => (p.col - 1, p.row + 1)
            })
    }

    fn neighbors(p: Position) -> Vec<Position> {
        HexDir::into_enum_iter().map(|d| d.next(p)).collect()
    }
}

fn dir_seq(line: &str) -> Vec<HexDir> {
    let mut result = Vec::new();
    let mut tentative = None;
    line.chars().for_each(|c| {
        match c {
            's' | 'n' => tentative = Some(c),
            'e' | 'w' => {
                result.push(HexDir::from(tentative, c));
                tentative = None;
            },
            _ => panic!("Illegal character: '{}'", c)
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use HexDir::*;

    #[test]
    fn test_input_line() {
        [
            ("esenee", vec![E, Se, Ne, E]),
            ("esew", vec![E, Se, W]),
            ("nwwswee", vec![Nw, W, Sw, E, E])
        ].iter().for_each(|(s, v)| {
            assert_eq!(&dir_seq(s), v);
        });
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("in/day24_ex.txt").unwrap(), "10");
    }

    #[test]
    fn test_black_adj() {
        let floor = Floor::from("in/day24_ex.txt").unwrap();
        for (p,c) in floor.floor.iter() {
            println!("{:?}: {:?} ({:?})", p, c, HexDir::neighbors(*p).iter()
                .filter(|n| floor.floor.contains_key(*n))
                .map(|n| (*n, floor.color(*n)))
                .collect::<Vec<(Position,TileColor)>>());
        }
        let expected_black_adj = [((-3, -1), 0), ((-3, 3), 0), ((-2, -2), 2), ((-2, 0), 1), ((-1, -3), 2), ((-1, -1), 2), ((0, -4), 2), ((0, -2), 5), ((0, 0), 1), ((0, 4), 0), ((1, -3), 2), ((2, -4), 2), ((2, 2), 0), ((3, -3), 1)];
        assert_eq!(expected_black_adj.len(), 14);
        assert_eq!(expected_black_adj.iter()
                       .map(|((row, col),_)| floor.color(Position::from((*col, *row))))
                       .filter(|c| *c == TileColor::Black)
                       .count(),
                   10);
        expected_black_adj.iter()
            .for_each(|((row, col), target)| {
                assert_eq!(floor.num_black_adj(Position::from((*col, *row))), *target);
            })
    }

    #[test]
    fn test_floor_of_life() {
        let counts: Vec<_> =
            FloorOfLifeIter { next: Some(Floor::from("in/day24_ex.txt").unwrap())}
                .map(|floor| floor.count_color(TileColor::Black))
                .take(101)
                .collect();
        for (day, count) in [
            (1, 15), (2, 12), (3, 25), (4, 14), (5, 23), (6, 28), (7, 41), (8, 37), (9, 49), (10, 37),
            (20, 132), (30, 259), (40, 406), (50, 566), (60, 788), (70, 1106), (80, 1373), (90, 1844), (100, 2208)
        ].iter() {
            assert_eq!(counts[*day], *count);
        }
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("in/day24_ex.txt").unwrap(), "2208");
    }
}