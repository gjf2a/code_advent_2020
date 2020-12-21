use std::fmt::Display;
use smallvec::SmallVec;
use smallvec::alloc::fmt::Formatter;
use std::{fmt, io};
use std::collections::BTreeMap;
use advent_code_lib::{all_lines, ManhattanDir, Position};
use enum_iterator::IntoEnumIterator;

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

    fn updated_pixels<F:Fn(usize,usize)->char>(&self, func: F) -> SmallVec<[SmallVec<[char;10]>;10]> {
        (0..self.height())
            .map(|y| (0..self.width())
                .map(|x| func(x, y))
                .collect())
            .collect()
    }

    fn rotated(&self, r: Rotation) -> Self {
        let mut result = self.clone();
        for _ in 0..match r {
            Rotation::R0 => 0,
            Rotation::R90 => 1,
            Rotation::R180 => 2,
            Rotation::R270 => 3,
        } {
            result.pixels = result.updated_pixels(|x, y| result.pixels[result.width() - x - 1][y]);
        }
        result
    }

    fn flipped(&self, f: Flip) -> Self {
        let mut result = self.clone();
        match f {
            Flip::X | Flip::Xy =>
                result.pixels = self.updated_pixels(|x, y| result.pixels[result.height() - y - 1][x]),
            _ => {}
        }
        match f {
            Flip::Y | Flip::Xy =>
                result.pixels = self.updated_pixels(|x, y| result.pixels[y][result.width() - x - 1]),
            _ => {}
        }
        result
    }

    fn edge(&self, side: ManhattanDir) -> String {
        match side {
            ManhattanDir::N => self.pixels[0].iter().collect(),
            ManhattanDir::S => self.pixels.last().unwrap().iter().collect(),
            ManhattanDir::E => (0..self.height()).map(|i| self.pixels[i].last().unwrap()).collect(),
            ManhattanDir::W => (0..self.height()).map(|i| self.pixels[i][0]).collect()
        }
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

#[derive(Debug,Clone,Copy,Eq,PartialEq,Ord,PartialOrd,IntoEnumIterator)]
enum Rotation {
    R0, R90, R180, R270
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Ord,PartialOrd,IntoEnumIterator)]
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

#[derive(Debug)]
struct Constraints {
    variants: BTreeMap<i64,BTreeMap<(Rotation,Flip), Tile>>,
    edges2variants: BTreeMap<(String,ManhattanDir),Vec<(i64,Rotation,Flip)>>
}

impl Constraints {
    fn new(pp: &PuzzlePieces) -> Self {
        let mut result = Constraints {variants: BTreeMap::new(), edges2variants: BTreeMap::new()};
        result.setup(pp);
        result.find_compatible();
        result
    }

    fn setup(&mut self, pp: &PuzzlePieces) {
        for (id, tile) in pp.tiles.iter() {
            self.variants.insert(*id, BTreeMap::new());
            for r in Rotation::into_enum_iter() {
                for f in Flip::into_enum_iter() {
                    self.variants.get_mut(id).unwrap().insert((r, f), tile.rotated(r).flipped(f));
                }
            }
        }
    }

    fn find_compatible(&mut self) {
        for (id, vars) in self.variants.iter() {
            for ((r, f), tile) in vars.iter() {
                for d in ManhattanDir::into_enum_iter() {
                    let key = (tile.edge(d), d);
                    let value = (*id, *r, *f);
                    match self.edges2variants.get_mut(&key) {
                        None => {self.edges2variants.insert(key, vec![value]);},
                        Some(v) => v.push(value)
                    }
                }
            }
        }
    }

    fn get_variant(&self, id: i64, r: Rotation, f: Flip) -> &Tile {
        self.variants.get(&id).unwrap().get(&(r, f)).unwrap()
    }
}

#[derive(Clone)]
struct Layout {
    tiles: BTreeMap<Position, (i64,Rotation,Flip)>,
    side: usize
}

impl Layout {
    fn new(pp: &PuzzlePieces) -> Self {
        let constraints = Constraints::new(pp);
        for (id, options) in constraints.variants.iter() {
            for ((r, f), tile) in options.iter() {
                let start = Layout {tiles: BTreeMap::new(), side: (pp.tiles.len() as f64).sqrt() as usize };
                if let Some(result) = start.find_assignment(&constraints,Position::new(), *id, *r, *f) {
                    return result;
                }
            }
        }
        panic!("No assignment possible");
    }

    fn in_bounds(&self, p: Position) -> bool {
        p.col < self.side as isize && p.row < self.side as isize
    }

    fn complete(&self) -> bool {
        self.tiles.len() == self.side.pow(2)
    }

    fn find_assignment(&self, constraints: &Constraints, assign: Position, id: i64, r: Rotation, f: Flip) -> Option<Layout> {
        if self.above_okay(constraints, assign, id, r, f) {
            let mut candidate = self.clone();
            candidate.tiles.insert(assign, (id, r, f));
            let mut assign = assign;
            let mut next_dir = ManhattanDir::E;
            let mut next = ManhattanDir::E.next(assign);
            if !self.in_bounds(next) {
                next_dir = ManhattanDir::S;
                assign = Position::from((0, assign.row));
                next = ManhattanDir::S.next(assign);
            }
            if self.in_bounds(next) {
                self.find_best_successor(constraints, constraints.get_variant(id, r, f).edge(next_dir), next, next_dir)
            } else {
                Some(candidate)
            }
        } else {
            None
        }
    }

    fn above_okay(&self, constraints: &Constraints, assign: Position, id: i64, r: Rotation, f: Flip) -> bool {
        let above_pos = ManhattanDir::N.next(assign);
        match self.tiles.get(&above_pos) {
            None => true,
            Some((id_up, r_up, f_up)) => {
                let edge_above = constraints.get_variant(id, r, f).edge(ManhattanDir::N);
                let edge_below = constraints.get_variant(*id_up, *r_up, *f_up).edge(ManhattanDir::S);
                edge_above == edge_below
            }
        }
    }

    fn find_best_successor(&self, constraints: &Constraints, edge: String, next: Position, next_dir: ManhattanDir) -> Option<Layout> {
        match constraints.edges2variants.get(&(edge, next_dir.inverse())) {
            None => None,
            Some(options) => {
                for (i,r,f) in options.iter() {
                    if let Some(success) = self.find_assignment(constraints, next, *i, *r, *f) {
                        return Some(success)
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_code_lib::ManhattanDir;

    #[test]
    fn load_ex() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        let nums = [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079];
        assert_eq!(pp.tiles.len(), nums.len());
        assert!(nums.iter().all(|num| pp.tiles.contains_key(num)));
        let constraints = Constraints::new(&pp);
        println!("{:?}", constraints.edges2variants);
    }

    #[test]
    fn load_puzzle() {
        let pp = PuzzlePieces::from("in/day20.txt").unwrap();
        let nums: Vec<_> = pp.tiles.keys().copied().collect();
        println!("{:?}", nums);
        println!("total: {}", nums.len())
    }

    fn strs_to_tiles<'a>(strs: &'a [&'a str]) -> impl Iterator<Item=Tile> + 'a {
        strs.iter().map(|s| str_to_tile(s))
    }

    fn str_to_tile<'a>(s: &'a str) -> Tile {
        Tile::from(&mut s.lines().map(|s| s.to_string())).unwrap()
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

    #[test]
    fn edge() {
        let tile = str_to_tile("Tile 1101:\n##.\n##.\n#.#\n");
        for (dir, target) in &[(ManhattanDir::N, "##."), (ManhattanDir::S, "#.#"), (ManhattanDir::E, "..#"), (ManhattanDir::W, "###")] {
            assert_eq!(&tile.edge(*dir).as_str(), target);
        }
    }
}