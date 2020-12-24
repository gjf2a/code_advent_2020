use advent_code_lib::{Position, all_lines};
use std::collections::BTreeMap;
use std::io;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Floor::from(filename)?.count_color(TileColor::Black).to_string())
}

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
                Some(color) => {*color = color.flip();}
            }
        });
        Ok(result)
    }

    fn count_color(&self, color: TileColor) -> usize {
        self.floor.values().filter(|v| **v == color).count()
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum TileColor {
    White, Black
}

impl TileColor {
    fn flip(&self) -> TileColor {
        match self {TileColor::Black => TileColor::White, TileColor::White => TileColor::Black}
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
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
}