use std::fmt::Display;
use smallvec::SmallVec;
use smallvec::alloc::fmt::Formatter;
use std::{fmt, io};
use std::collections::{BTreeMap, BTreeSet};
use advent_code_lib::{all_lines, ManhattanDir, Position};
use enum_iterator::IntoEnumIterator;
use itertools::Itertools;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(PuzzlePieces::from(filename)?.corner_product().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let (count, monsters) = find_monsters(filename)?;
    println!("{}", monsters);
    Ok(count.to_string())
}

fn find_monsters(filename: &str) -> io::Result<(usize, Tile)> {
    let pp = PuzzlePieces::from(filename)?;
    Ok(Layout::from(&pp).image(&pp).all_variants().iter()
        .map(|tile| {
            let mut tile = tile.clone();
            tile.show_monsters();
            (tile.count('#'), tile)
        })
        .min().unwrap())
}

#[derive(Clone,Debug,Eq,PartialEq,Ord,PartialOrd)]
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

    fn all_variants(&self) -> Vec<Tile> {
        Rotation::into_enum_iter()
            .cartesian_product(Flip::into_enum_iter())
            .map(|(r, f)| self.rotated(r).flipped(f))
            .collect()
    }

    fn count(&self, t: char) -> usize {
        self.pixels.iter().map(|row| row.iter().filter(|c| **c == t).count()).sum()
    }

    fn sea_monster() -> Self {
        let mut monster_lines = "Tile: 13\n                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ".lines().map(|s| s.to_string());
        Tile::from(&mut monster_lines).unwrap()
    }

    fn show_monsters(&mut self) {
        let template = Tile::sea_monster();
        for col in 0..self.width() {
            for row in 0..self.height() {
                self.blot_at(&template, col, row);
            }
        }
    }

    fn blot_at(&mut self, subimage: &Tile, col: usize, row: usize) {
        if col + subimage.width() >= self.width() || row + subimage.height() >= self.height() {
            return;
        }
        let mut updates = Vec::new();
        for sub_col in 0..subimage.width() {
            for sub_row in 0..subimage.height() {
                if subimage.get(sub_col, sub_row) == '#' {
                    if self.get(col + sub_col, row + sub_row) == '#' {
                        updates.push((col + sub_col, row + sub_row));
                    } else {
                        return;
                    }
                }
            }
        }
        for (c, r) in updates {
            self.pixels[r][c] = 'O';
        }
    }

    fn get(&self, col: usize, row: usize) -> char {
        self.pixels[row][col]
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
    tiles: BTreeMap<i64,Tile>,
    tile_height: usize,
    tile_width: usize
}

impl PuzzlePieces {
    fn from(filename: &str) -> io::Result<Self> {
        let mut tiles = BTreeMap::new();
        let mut tile_width = 0;
        let mut tile_height = 0;
        let mut lines = all_lines(filename)?;
        loop {
            match Tile::from(&mut lines) {
                None => break,
                Some(tile) => {
                    tile_width = tile.width();
                    tile_height = tile.height();
                    tiles.insert(tile.id, tile);
                }
            }
        }
        Ok(PuzzlePieces { tiles, tile_height, tile_width })
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
            self.variants.insert(*id, Rotation::into_enum_iter().cartesian_product(Flip::into_enum_iter()).map(|(r, f)| ((r, f), tile.rotated(r).flipped(f))).collect());
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

    fn create_match(&mut self, v: TileVariant) -> Option<(TileVariant, ManhattanDir)> {
        for dir in ManhattanDir::into_enum_iter() {
            let edge2next = self.get_variant(v).edge(dir);
            for m in self.edges2variants.get(&(edge2next, dir.inverse())).unwrap().iter() {
                if !self.assigned.contains(&m.id) {
                    self.assigned.insert(m.id);
                    return Some((*m, dir));
                }
            }
        }
        None
    }

    // This function is used in a test to show that the problem input is restricted to no more
    // than two tiles sharing any given edge.
    #[allow(dead_code)]
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
    tiles: BTreeMap<Position, TileVariant>,
    tile_width: usize,
    tile_height: usize,
    tile_columns: usize,
    tile_rows: usize
}

impl Layout {
    fn new(tiles: BTreeMap<Position,TileVariant>, tile_width: usize, tile_height: usize) -> Self {
        let (min, _) = tiles.first_key_value().unwrap();
        let (max, _) = tiles.last_key_value().unwrap();
        let diff = *max - *min;
        Layout {tiles: tiles.iter().map(|(p,tv)| (*p - *min, *tv)).collect(),
            tile_columns: (diff.col + 1) as usize, tile_rows: (diff.row + 1) as usize,
            tile_width: tile_width - 2, tile_height: tile_height - 2}
    }

    fn from(pp: &PuzzlePieces) -> Self {
        let mut constraints = Constraints::new(pp);
        let mut selected = TileVariant {id: *pp.corner_ids().first().unwrap(), rotation: Rotation::R0, flip: Flip::Id};
        constraints.assign(selected.id);
        let mut tiles = BTreeMap::new();
        let mut p = Position::new();
        loop {
            tiles.insert(p, selected);
            let (next, next_dir) = match constraints.create_match(selected) {
                Some(next) => next,
                None => return Layout::new(tiles, pp.tile_width, pp.tile_height)
            };
            p = next_dir.next(p);
            selected = next;
        }
    }

    fn char_at(&self, pp: &PuzzlePieces, col: usize, row: usize) -> char {
        let tile_col = (col / self.tile_width) as isize;
        let tile_row = (row / self.tile_height) as isize;
        let tile_id = self.tiles.get(&Position::from((tile_col, tile_row))).unwrap();
        let char_col = col % self.tile_width + 1;
        let char_row = row % self.tile_height + 1;
        pp.tiles.get(&tile_id.id).unwrap().rotated(tile_id.rotation).flipped(tile_id.flip).get(char_col, char_row)
    }

    fn image(&self, pp: &PuzzlePieces) -> Tile {
        Tile {id: 0, pixels: (0..self.tile_rows * self.tile_height)
            .map(|r| (0..self.tile_columns * self.tile_width)
                .map(|c| self.char_at(pp, c, r))
                .collect())
            .collect()}
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut prev: Option<Position> = None;
        for (p,n) in self.tiles.iter() {
            if let Some(prev) = prev {
                if prev.row != p.row {
                    writeln!(f).unwrap();
                }
            }
            write!(f, "{} ", n.id).unwrap();
            prev = Some(*p);
        }
        writeln!(f)
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

    fn str_to_tile(s: &str) -> Tile {
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
    fn layout_example() {
        let pp = PuzzlePieces::from("in/day20_ex.txt").unwrap();
        let layout = Layout::from(&pp);
        assert_eq!(layout.tiles.len(), 9);
        assert_eq!(format!("{}", layout), "3079 2311 1951 \n2473 1427 2729 \n1171 1489 2971 \n");
        assert_eq!(format!("{}", layout.image(&pp)), "Tile 0:
#####..##.#...##.#..#.#.
......#..#....#.#....###
...######..#.#.###.##.##
#..#.#####.#...#####.###
##.#...####.##.#....#.##
#.#####....#.########...
..###.#.#..##...#..#....
......#.....#..#...####.
....##.#.###..#..##.#..#
..###.#.#.####.#..####.#
##..#.######.#...#.#.###
#.########..##....####.#
..#.#.#.#...#...#.##..##
###.###..##.#.#..#..#...
.##.###...#.##.#....#.#.
..######.##.#..#...#.###
..##.#..#.##.##.###.#.#.
#.#..#.###...#.###.####.
###.####.#.#.#..#..#.#..
.###.###.#.#.#...####..#
##....###...#####..#####
#...####..#...#..#..##.#
.##.####..##..##..###.#.
###..#...#...##...###...
");
    }

    #[test]
    fn layout_real() {
        let pp = PuzzlePieces::from("in/day20.txt").unwrap();
        let layout = Layout::from(&pp);
        assert_eq!(layout.tiles.len(), 144);
        assert_eq!(format!("{}", layout), "3389 3169 2591 1511 1901 2467 1777 1667 2797 3449 2861 1657 \n3461 2179 3391 1607 1487 1297 2609 3923 3931 3697 3559 1049 \n1327 3659 3011 1217 1423 2503 1303 2111 1061 2441 2897 2389 \n3719 3253 3491 2251 2399 2789 2543 3413 3797 1051 1163 2381 \n1877 3257 1549 2887 1949 1447 3821 1619 1483 1319 1571 3947 \n2801 1109 2099 1231 1381 1367 2137 2677 2311 1579 3323 2729 \n3761 3319 2833 3187 2663 1697 3889 1583 3023 1489 1741 3583 \n3853 2213 2351 3581 1409 1427 3733 2741 2557 3271 1693 1973 \n1009 1087 1559 2011 1997 3299 1789 3301 3593 3163 1093 3767 \n2113 2879 3229 3313 1277 3863 3623 2837 1747 3191 1123 1709 \n2963 2767 3617 1907 3331 2939 3527 2081 1279 1091 2339 1021 \n1621 3793 1879 3709 1181 3881 2593 1801 1307 3541 3727 3547 \n");
    }

    #[test]
    fn monster() {
        let (count, monsters) = find_monsters("in/day20_ex.txt").unwrap();
        let found = "Tile 0:
.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.O#..
#.O.##.OO#.#.OO.##.OOO##
..#O.#O#.O##O..O.#O##.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.##O###.
.O##.#OO.###OO##..OOO##.
..O#.O..O..O.#O##O##.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#
";
        assert_eq!(format!("{}", monsters), found);
        assert_eq!(count, 273);
    }
}