use std::fmt::Display;
use smallvec::SmallVec;
use smallvec::alloc::fmt::Formatter;
use std::{fmt, io};
use std::collections::BTreeMap;
use advent_code_lib::all_lines;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Tile {
    id: i64,
    pixels: SmallVec<[SmallVec<[char; 10]>; 10]>
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id).unwrap();
        for row in self.pixels.iter() {
            writeln!(f, "{}", row.iter().collect::<String>()).unwrap();
        }
        Ok(())
    }
}

fn indices_2d_smallvec<T,F:Fn(usize,usize)->T>(width: usize, height: usize, func: F) -> SmallVec<[SmallVec<[T;10]>;10]> {
    (0..height)
        .map(|y| (0..width)
            .map(|x| func(x, y))
            .collect())
        .collect()
}

impl Tile {
    fn from<I:Iterator<Item=String>>(lines: &mut I) -> Option<Self> {
        lines.next().map(|header| {
            let id = header.split_whitespace().skip(1).next().unwrap().split(':').next().unwrap().parse::<i64>().unwrap();
            let pixels = lines
                .take_while(|line| line.len() > 0)
                .map(|line| line.chars().collect())
                .collect();
            Tile {id, pixels}
        })
    }

    fn height(&self) -> usize {
        self.pixels.len()
    }

    fn width(&self) -> usize {
        self.pixels[0].len()
    }

    fn rotated(&self, r: Rotation) -> Self {
        let mut result = self.clone();
        for _ in 0..match r {
            Rotation::R0 => 0,
            Rotation::R90 => 1,
            Rotation::R180 => 2,
            Rotation::R270 => 3,
        } {
            result.pixels = indices_2d_smallvec(result.width(), result.height(), |x, y| result.pixels[result.width() - x - 1][y]);
        }
        result
    }

    fn flipped(&self, f: Flip) -> Self {
        let mut result = self.clone();
        match f {
            Flip::X | Flip::Xy =>
                result.pixels = indices_2d_smallvec(result.width(), result.height(), |x, y| result.pixels[result.height() - y - 1][x]),
            _ => {}
        }
        match f {
            Flip::Y | Flip::Xy => result.pixels = indices_2d_smallvec(result.width(), result.height(), |x, y| result.pixels[y][result.width() - x - 1]),
            _ => {}
        }
        result
    }
}

#[derive(Debug,Clone)]
struct PuzzlePieces {
    tiles: BTreeMap<i64,Tile>
}

impl PuzzlePieces {
    fn from(filename: &str) -> io::Result<Self> {
        let mut pp = PuzzlePieces { tiles: BTreeMap::new()};
        let mut lines = all_lines(filename)?;
        loop {
            match Tile::from(&mut lines) {
                None => break,
                Some(tile) => {pp.tiles.insert(tile.id, tile);}
            }
        }
        Ok(pp)
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Rotation {
    R0, R90, R180, R270
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Flip {
    Id, X, Y, Xy
}

impl Display for PuzzlePieces {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for tile in self.tiles.values() {
            writeln!(f, "{}", tile).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        let nums = [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079];
        assert_eq!(pp.tiles.len(), nums.len());
        assert!(nums.iter().all(|num| pp.tiles.contains_key(num)));
    }

    fn strs_to_tiles<'a>(strs: &'a [&'a str]) -> impl Iterator<Item=Tile> + 'a {
        strs.iter().map(|s| Tile::from(&mut s.lines().map(|s| s.to_string())).unwrap())
    }

    #[test]
    fn rotate() {
        let tiles: Vec<(Tile,Rotation)> = strs_to_tiles(&[
            "Tile 1101:\n###\n...\n#.#\n",
            "Tile 1101:\n#.#\n..#\n#.#\n",
            "Tile 1101:\n#.#\n...\n###\n",
            "Tile 1101:\n#.#\n#..\n#.#\n"])
            .zip(&[Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270])
            .map(|(t, r)| (t, *r))
            .collect();
        let (start,_) = &(tiles[0]);
        for (tile, rotation) in tiles.iter() {
            assert_eq!(&start.rotated(*rotation), tile);
        }
    }

    #[test]
    fn flip() {
        let tiles: Vec<(Tile,Flip)> = strs_to_tiles(&[
            "Tile 1101:\n##.\n...\n#.#\n",
            "Tile 1101:\n#.#\n...\n##.\n",
            "Tile 1101:\n.##\n...\n#.#\n",
            "Tile 1101:\n#.#\n...\n.##\n"])
            .zip(&[Flip::Id, Flip::X, Flip::Y, Flip::Xy])
            .map(|(t, f)| (t, *f))
            .collect();
        let (start,_) = &(tiles[0]);
        for (tile, flip) in tiles.iter() {
            assert_eq!(&start.flipped(*flip), tile);
        }
    }
}