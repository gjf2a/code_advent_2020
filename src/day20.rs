use std::fmt::Display;
use smallvec::SmallVec;
use smallvec::alloc::fmt::Formatter;
use std::{fmt, io};
use std::collections::{BTreeMap, BTreeSet};
use advent_code_lib::{all_lines, ManhattanDir, Position};
use enum_iterator::IntoEnumIterator;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(PuzzlePieces::from(filename)?.corner_product().to_string())
}

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

    fn all_possible_edges(&self) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        for dir in ManhattanDir::into_enum_iter() {
            let edge = self.edge(dir);
            result.insert(edge.chars().rev().collect());
            result.insert(edge);
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

    fn corner_ids(&self) -> Vec<i64> {
        let edges = Edges::from(self);
        self.tiles.iter()
            .filter(|(_, tile)| edges.edges_with_friends(tile) == 2)
            .map(|(id,_)| *id)
            .collect()
    }

    fn corner_product(&self) -> i64 {
        self.corner_ids().iter().product()
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

struct Edges {
    edge2tile: BTreeMap<String,Vec<i64>>
}

impl Edges {
    fn from(pp: &PuzzlePieces) -> Self {
        let mut result = Edges { edge2tile: BTreeMap::new() };
        for (id, tile) in pp.tiles.iter() {
            for edge in tile.all_possible_edges() {
                result.add(edge, *id);
            }
        }
        result
    }

    fn add(&mut self, edge: String, id: i64) {
        if let Some(v) = self.edge2tile.get_mut(edge.as_str()) {
            v.push(id);
        } else {
            self.edge2tile.insert(edge, vec![id]);
        }
    }

    fn edges_with_friends(&self, tile: &Tile) -> usize {
        ManhattanDir::into_enum_iter()
            .map(|d| tile.edge(d))
            .filter(|e| self.edge2tile.get(e.as_str()).unwrap().len() > 1 || self.edge2tile.get(e.chars().rev().collect::<String>().as_str()).unwrap().len() > 1)
            .count()
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

#[derive(Clone, Debug)]
struct Layout {
    tiles: BTreeMap<Position, (i64,Rotation,Flip)>,
    ids: BTreeSet<i64>,
    side: usize
}

impl Layout {
    fn new(pp: &PuzzlePieces) -> Self {
        let constraints = Constraints::new(pp);
        for (id, options) in constraints.variants.iter() {
            for (r, f) in options.keys() {
                let start = Layout {tiles: BTreeMap::new(), ids: BTreeSet::new(), side: (pp.tiles.len() as f64).sqrt() as usize };
                if let Some(result) = start.find_assignment(&constraints,Position::new(), *id, *r, *f) {
                    return result;
                }
            }
        }
        panic!("No assignment possible");
    }

    fn corner_id_product(&self) -> i64 {
        let (corner1, (id1, _, _)) = self.tiles.first_key_value().unwrap();
        let (corner2, (id2, _, _)) = self.tiles.last_key_value().unwrap();
        let corner3 = Position::from((corner1.col, corner2.row));
        let corner4 = Position::from((corner2.col, corner1.row));
        let (id3, _, _) = self.tiles.get(&corner3).unwrap();
        let (id4, _, _) = self.tiles.get(&corner4).unwrap();
        id1 * id2 * id3 * id4
    }

    fn print_id_layout(&self) {
        for row in 0..self.side {
            for col in 0..self.side {
                let p = Position::from((col as isize, row as isize));
                if let Some((id, _, _)) = self.tiles.get(&p) {
                    print!("{} ", id);
                }
            }
            println!();
        }
        println!();
    }

    fn find_assignment(&self, constraints: &Constraints, assign: Position, id: i64, r: Rotation, f: Flip) -> Option<Layout> {
        if self.above_okay(constraints, assign, id, r, f) {
            let mut candidate = self.clone();
            candidate.tiles.insert(assign, (id, r, f));
            candidate.ids.insert(id);
            let (prev, next, next_dir) = self.square_prev_next_dir(assign);
            if next.row < self.side as isize {
                let (ni, nr, nf) = candidate.tiles.get(&prev).unwrap();
                candidate.find_best_successor(constraints, constraints.get_variant(*ni, *nr, *nf).edge(next_dir), next, next_dir)
            } else {
                Some(candidate)
            }
        } else {
            None
        }
    }

    fn square_prev_next_dir(&self, current: Position) -> (Position, Position, ManhattanDir) {
        let (dir, pos) = if current.col == self.side as isize - 1 {
            (ManhattanDir::S, Position::from((0, current.row)))
        } else {
            (ManhattanDir::E, current)
        };
        (pos, dir.next(pos), dir)
    }

    fn above_okay(&self, constraints: &Constraints, assign: Position, id: i64, r: Rotation, f: Flip) -> bool {
        let above_pos = ManhattanDir::N.next(assign);
        match self.tiles.get(&above_pos) {
            None => true,
            Some((id_up, r_up, f_up)) => {
                let edge_below = constraints.get_variant(*id_up, *r_up, *f_up).edge(ManhattanDir::S);
                let edge_above = constraints.get_variant(id, r, f).edge(ManhattanDir::N);
                edge_above == edge_below
            }
        }
    }

    fn find_best_successor(&self, constraints: &Constraints, edge: String, next: Position, next_dir: ManhattanDir) -> Option<Layout> {
        match constraints.edges2variants.get(&(edge, next_dir.inverse())) {
            None => None,
            Some(options) => {
                for (i,r,f) in options.iter() {
                    if !self.ids.contains(i) {
                        if let Some(success) = self.find_assignment(constraints, next, *i, *r, *f) {
                            return Some(success)
                        }
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

    #[test]
    fn puzzle1() {
        assert_eq!(solve_1("in/day20_ex.txt").unwrap(), "20899048083289");
    }

    #[test]
    fn edges_friends() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        println!("{:?}", pp.corner_ids());/*
        println!("Hello!!!!");
        let edges = Edges::from(&pp);
        for id in [1951, 3079, 2971, 1171].iter() {
            assert_eq!(edges.edges_with_friends(pp.tiles.get(id).unwrap()), 2);
        }
        for (id, tile) in pp.tiles.iter() {
            println!("{}: {} edges with friends", id, edges.edges_with_friends(tile));
        }*/
    }
}