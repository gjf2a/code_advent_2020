use advent_code_lib::for_each_line;
use std::io;
use std::collections::{BTreeSet, BTreeMap};
use std::collections::btree_map::Keys;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let graph = create_graph_from(filename)?;
    let total = graph.all_node_names()
        .filter(|name| graph.all_successors_of(name).contains("shiny gold"))
        .count();
    Ok(total.to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let graph = create_graph_from(filename)?;
    Ok(graph.bags_within("shiny gold").to_string())
}

fn create_graph_from(filename: &str) -> io::Result<StringGraph> {
    let mut graph = StringGraph::new();
    for_each_line(filename, |line| Ok({
        let left_right: Vec<&str> = line.split("contain ").collect();
        let key = bag_color(left_right[0]);
        if !left_right[1].contains("no other bags") {
            left_right[1].split(", ")
                .map(|s| bag_count_and_color(s))
                .for_each(|(count, color)| {
                    graph.add_edge(key.as_str(), color.as_str(), count);
                });
        }
    }))?;
    Ok(graph)
}

fn bag_color(bag_src: &str) -> String {
    bag_src.split(" bag").next().unwrap().to_string()
}

fn bag_count_and_color(bag_src: &str) -> (usize, String) {
    let bag_parts: Vec<&str> = bag_src.splitn(2, ' ').collect();
    (bag_parts[0].parse::<usize>().unwrap(), bag_color(bag_parts[1]))
}

#[derive(Clone,Debug,Eq,Ord,PartialOrd,PartialEq)]
pub struct StringGraph {
    node2nodes: BTreeMap<String,BTreeMap<String,usize>>
}

impl StringGraph {
    pub fn new() -> Self {StringGraph {node2nodes: BTreeMap::new()}}

    pub fn add_if_absent(&mut self, name: &str) {
        if !self.node2nodes.contains_key(name) {
            self.node2nodes.insert(name.to_string(), BTreeMap::new());
        }
    }

    pub fn add_edge(&mut self, start: &str, end: &str, count: usize) {
        self.add_if_absent(start);
        self.add_if_absent(end);
        self.node2nodes.get_mut(start).unwrap().insert(end.to_string(), count);
    }

    pub fn all_node_names(&self) -> Keys<String,BTreeMap<String,usize>> {
        self.node2nodes.keys()
    }

    pub fn count_from(&self, start: &str, end: &str) -> usize {
        self.node2nodes.get(start)
            .map_or(0, |m|
                m.get(end).map_or(0, |s| *s))
    }

    pub fn all_successors_of(&self, name: &str) -> BTreeSet<String> {
        let mut visited = BTreeSet::new();
        if self.node2nodes.contains_key(name) {
            let mut open_list: Vec<String> = self.node2nodes.get(name).unwrap().keys()
                .map(|s| s.clone())
                .collect();
            while open_list.len() > 0 {
                let candidate = open_list.pop().unwrap();
                if !visited.contains(candidate.as_str()) {
                    self.node2nodes.get(candidate.as_str()).unwrap().keys()
                        .for_each(|s| open_list.push(s.clone()));
                    visited.insert(candidate);
                }
            }
        }
        visited
    }

    pub fn bags_within(&self, name: &str) -> usize {
        self.node2nodes.get(name)
            .map_or(0,
                    |m| m.iter()
                        .map(|(key,value)| value * (1 + self.bags_within(key)))
                        .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_string_graph() {
        let mut sg = StringGraph::new();
        [("a", "b"), ("b", "c"), ("c", "d"), ("b", "e")].iter()
            .for_each(|(a, b)| sg.add_edge(*a, *b, 1));
        [("a", btreeset!("b", "c", "d", "e")), ("b", btreeset!("c", "d", "e")),
            ("c", btreeset!("d")), ("d", btreeset!()), ("e", btreeset!())].iter()
            .for_each(|(k, s)| {
                assert_eq!(sg.all_successors_of(k),
                           s.iter().map(|s| s.to_string()).collect::<BTreeSet<String>>());
            });
    }

    #[test]
    pub fn test_bag_color() {
        [("light red bags", "light red"), ("bright white bag", "bright white"),
            ("muted yellow bags", "muted yellow")].iter()
            .for_each(|(b, c)| {
            assert_eq!(bag_color(b).as_str(), *c);
        });
    }

    #[test]
    pub fn test_bag_count() {
        [("1 bright white bag", 1, "bright white"),
            ("2 muted yellow bags", 2, "muted yellow"),
            ("3 bright white bags", 3, "bright white"),
            ("4 muted yellow bags", 4, "muted yellow"),
            ("1 shiny gold bag", 1, "shiny gold"),
            ("2 shiny gold bags", 2, "shiny gold"),
            ("9 faded blue bags", 9, "faded blue")].iter()
            .for_each(|(src, count, color)| {
                let (ct, cl) = bag_count_and_color(src);
                assert_eq!(cl.as_str(), *color);
                assert_eq!(ct, *count as usize);
            });
    }

    #[test]
    pub fn test_create_example() {
        let graph = create_graph_from("day_7_example.txt").unwrap();
        [("light red", "bright white", 1),
            ("light red", "muted yellow", 2),
            ("dark orange", "bright white", 3),
            ("dark orange", "muted yellow", 4),
            ("bright white", "shiny gold", 1),
            ("muted yellow", "shiny gold", 2),
            ("shiny gold", "dark olive", 1),
            ("shiny gold", "vibrant plum", 2),
            ("dark olive", "faded blue", 3),
            ("dark olive", "dotted black", 4),
            ("vibrant plum", "faded blue", 5),
            ("vibrant plum", "dotted black", 6)].iter()
            .for_each(|(start, end, count)| {
            assert_eq!(graph.count_from(*start, *end), *count as usize);
        });
    }

    #[test]
    pub fn test_solve_1() {
        assert_eq!(solve_1("day_7_example.txt").unwrap(), "4");
    }

    #[test]
    pub fn test_bag_sum() {
        let graph = create_graph_from("day_7_example.txt").unwrap();
        [("faded blue", 0), ("dotted black", 0), ("vibrant plum", 11), ("dark olive", 7),
            ("shiny gold", 32), ("muted yellow", 75)].iter()
            .for_each(|(color, count)| {
                assert_eq!(graph.bags_within(color), *count as usize)
            });
    }

    #[test]
    pub fn test_solve_2_1() {
        assert_eq!(solve_2("day_7_example.txt").unwrap(), "32");
    }

    #[test]
    pub fn test_solve_2_2() {
        assert_eq!(solve_2("day_7_example_2.txt").unwrap(), "126");
    }
}