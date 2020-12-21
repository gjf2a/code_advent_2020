use std::collections::{BTreeMap, BTreeSet};
use histogram::Histogram;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Allergens::from(filename)?.allergen_free_counts().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(Allergens::from(filename)?.canonical_dangerous_list())
}

#[derive(Debug)]
struct AllergenCandidates {
    allergen2candidates: BTreeMap<String,BTreeSet<String>>,
    allergen2ingredient: BTreeMap<String,String>
}

impl AllergenCandidates {
    fn new() -> Self {
        AllergenCandidates { allergen2candidates: BTreeMap::new(), allergen2ingredient: BTreeMap::new() }
    }

    fn add(&mut self, allergen: &str, candidate_row: &Vec<String>) {
        let candidates = candidate_row.iter().map(|s| s.to_string()).collect();
        match self.allergen2candidates.get_mut(allergen) {
            None => {self.allergen2candidates.insert(allergen.to_string(), candidates);}
            Some(set) => {*set = set.intersection(&candidates).map(|s| s.clone()).collect();}
        }
    }

    fn single_candidate(&self) -> (String,String) {
        for (candidate, set) in self.allergen2candidates.iter() {
            if set.len() == 1 {
                return (candidate.clone(), set.first().unwrap().clone())
            }
        }
        panic!("This shouldn't happen")
    }

    fn purge_candidate(&mut self, candidate: &str) {
        for set in self.allergen2candidates.values_mut() {
            set.remove(candidate);
        }
    }

    fn reduce(&mut self) {
        while !self.allergen2candidates.is_empty() {
            let (allergen, ingredient) = self.single_candidate();
            self.allergen2candidates.remove(allergen.as_str());
            self.purge_candidate(ingredient.as_str());
            self.allergen2ingredient.insert(allergen, ingredient);
        }
    }
}

#[derive(Debug)]
struct Allergens {
    allergen2ingredient: BTreeMap<String,String>,
    ingredient2allergen: BTreeMap<String,String>,
    ingredient_counts: Histogram<String>
}

impl Allergens {
    fn from(filename: &str) -> io::Result<Self> {
        let mut allergen_search = AllergenCandidates::new();
        let mut ingredient_counts = Histogram::new();
        all_lines(filename)?.for_each(|line| {
            let (ingredients, allergens) = process_input_line(line);
            for allergen in allergens {
                allergen_search.add(allergen.as_str(), &ingredients);
            }
            for ingredient in ingredients {
                ingredient_counts.bump(&ingredient);
            }
        });
        allergen_search.reduce();
        let ingredient2allergen = allergen_search.allergen2ingredient.iter().map(|(a, i)| (i.clone(), a.clone())).collect();
        Ok(Allergens {allergen2ingredient: allergen_search.allergen2ingredient, ingredient2allergen, ingredient_counts})
    }

    fn allergen_free_counts(&self) -> usize {
        self.ingredient_counts.iter().filter(|(i,_)| !self.ingredient2allergen.contains_key(i.as_str())).map(|(i, c)| c).sum()
    }

    fn canonical_dangerous_list(&self) -> String {
        let ingredients: Vec<_> = self.allergen2ingredient.iter().map(|(_, ingredient)| ingredient.clone()).collect();
        ingredients.join(",")
    }
}

fn process_input_line(line: String) -> (Vec<String>, Vec<String>) {
    let mut parts1 = line.split(" (contains ");
    let ingredients = parts1.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let allergens = parts1.next().unwrap().split(&[',', ' ', ')'][..]).filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
    (ingredients, allergens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let allergens = Allergens::from("in/day21_ex.txt").unwrap();
        assert_eq!(allergens.allergen_free_counts(), 5);
        assert_eq!(allergens.canonical_dangerous_list(), "mxmxvkd,sqjhc,fvjkl");
        println!("{:?}", allergens);
    }
}