use std::fmt::Display;
use smallvec::SmallVec;
use smallvec::alloc::fmt::Formatter;
use std::{fmt, io};
use std::collections::{BTreeMap, BTreeSet};
use advent_code_lib::{all_lines, ManhattanDir, Position};
use enum_iterator::IntoEnumIterator;

// Puzzle 2 concept
// - Start by finding the corners and edges
// - Next, find the border by trying out permutations of corners and edges
// - Then, figure out the corners and edges of the next interior based on the exposed inner edges.
// - Use recursion to fill in the next ring.

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

    fn final_template(side: usize) -> Self {
        Tile {id: 0, pixels: (0..side).map(|_| (0..side).map(|_| ' ').collect()).collect()}
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

    fn side(&self) -> usize {
        (self.tiles.len() as f64).sqrt() as usize
    }

    fn ids_with_friends(&self, friends: usize) -> BTreeSet<i64> {
        let edges = Edges::from(self);
        self.tiles.iter()
            .filter(|(_, tile)| edges.edges_with_friends(tile) == friends)
            .map(|(id,_)| *id)
            .collect()
    }

    fn corner_ids(&self) -> BTreeSet<i64> {
        self.ids_with_friends(2)
    }

    fn edge_ids(&self) -> BTreeSet<i64> {
        self.ids_with_friends(3)
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

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct TileVariant {
    id: i64, rotation: Rotation, flip: Flip
}

#[derive(Debug)]
struct Constraints {
    variants: BTreeMap<i64,BTreeMap<(Rotation,Flip), Tile>>,
    edges2variants: BTreeMap<(String,ManhattanDir),Vec<TileVariant>>,
    assigned: BTreeSet<i64>
}

impl Constraints {
    fn new(pp: &PuzzlePieces) -> Self {
        let mut result = Constraints {variants: BTreeMap::new(), edges2variants: BTreeMap::new(), assigned: BTreeSet::new()};
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
            for ((rotation, flip), tile) in vars.iter() {
                for d in ManhattanDir::into_enum_iter() {
                    let key = (tile.edge(d), d);
                    let value = TileVariant {id: *id, rotation: *rotation, flip: *flip};
                    match self.edges2variants.get_mut(&key) {
                        None => {self.edges2variants.insert(key, vec![value]);},
                        Some(v) => v.push(value)
                    }
                }
            }
        }
    }

    fn assign(&mut self, id: i64) {
        self.assigned.insert(id);
    }

    fn get_variant(&self, v: TileVariant) -> &Tile {
        self.variants.get(&v.id).unwrap().get(&(v.rotation, v.flip)).unwrap()
    }

    fn get_match(&mut self, v: TileVariant, dir: ManhattanDir) -> Option<TileVariant> {
        let edge2next = self.get_variant(v).edge(dir);
        println!("v: {:?} edge2next: {}", v, edge2next);
        for m in self.edges2variants.get(&(edge2next, dir.inverse())).unwrap().iter() {
            println!("considering {:?}", m);
            if !self.assigned.contains(&m.id) {
                self.assigned.insert(m.id);
                println!("Success");
                return Some(*m);
            }
        }
        println!("Failed");
        None
    }

    fn edges2ids(&self) -> BTreeMap<String,BTreeSet<i64>> {
        let mut result: BTreeMap<String,BTreeSet<i64>> = BTreeMap::new();
        for ((edge,_),ids) in self.edges2variants.iter() {
            let ids_iter = ids.iter().map(|v| v.id);
            match result.get_mut(edge.as_str()) {
                None => {result.insert(edge.clone(), ids_iter.collect());}
                Some(set) => *set = set.union(&ids_iter.collect()).copied().collect()
            }
        }
        result
    }
}

#[derive(Debug)]
struct Layout {
    tiles: BTreeMap<Position, TileVariant>
}

impl Layout {
    fn from(pp: &PuzzlePieces) -> Self {
        let mut constraints = Constraints::new(pp);
        let mut selected = TileVariant {id: *pp.corner_ids().first().unwrap(), rotation: Rotation::R0, flip: Flip::Id};
        constraints.assign(selected.id);
        let mut next_dir = ManhattanDir::E;
        let mut result = Layout { tiles: BTreeMap::new() };
        let mut p = Position::new();
        loop {
            result.tiles.insert(p, selected);
            let next = match constraints.get_match(selected, next_dir) {
                Some(next) => next,
                None => {
                    next_dir = next_dir.clockwise();
                    match constraints.get_match(selected, next_dir) {
                        Some(next) => next,
                        None => return result
                    }
                }
            };
            p = next_dir.next(p);
            selected = next;
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
    fn max_2_ids_per_edge() {
        let constraints = Constraints::new(&PuzzlePieces::from("in/day20.txt").unwrap());
        let mut counts = BTreeSet::new();
        for (_, id_set) in constraints.edges2ids() {
            counts.insert(id_set.len());
        }
        assert_eq!(counts, btreeset! {1, 2});
    }

    #[test]
    fn example_corners() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        assert_eq!(pp.corner_ids(), btreeset! {1171, 1951, 2971, 3079});
    }

    #[test]
    fn layout() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        let layout = Layout::from(&pp);
        println!("{:?}", layout);
    }
}