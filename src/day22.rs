use std::collections::{VecDeque, BTreeSet};
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (mut deck1, mut deck2) = decks_from(filename)?;
    Ok(play_puzzle_1_to_end(&mut deck1, &mut deck2).to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let (mut deck1, mut deck2) = decks_from(filename)?;
    let mut game = Puzzle2Game::new();
    Ok(game.player_1_wins(&mut deck1, &mut deck2).1.to_string())
}

fn decks_from(filename: &str) -> io::Result<(Deck,Deck)> {
    let mut iter = all_lines(filename)?;
    let deck1 = Deck::from(&mut iter);
    let deck2 = Deck::from(&mut iter);
    Ok((deck1, deck2))
}

#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Deck {
    player: String,
    cards: VecDeque<usize>
}

impl Deck {
    fn from<I:Iterator<Item=String>>(iter: &mut I) -> Self {
        let player = iter.next().unwrap();
        let cards = iter
            .take_while(|s| s.len() > 0)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Deck {player, cards}
    }

    fn copy_n(&self, n: usize) -> Self {
        Deck {player: self.player.clone(), cards: self.cards.iter().take(n).copied().collect()}
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn score(&self) -> usize {
        self.cards.iter().rev().enumerate()
            .map(|(count, value)| (count + 1) as usize * value)
            .sum()
    }
}

fn play_puzzle_1_one_round(deck1: &mut Deck, deck2: &mut Deck) {
    let card1 = deck1.cards.pop_front().unwrap();
    let card2 = deck2.cards.pop_front().unwrap();
    resolve_winner(card1 > card2, card1, card2, deck1, deck2);
}

fn resolve_winner(one_wins: bool, card1: usize, card2: usize, deck1: &mut Deck, deck2: &mut Deck) {
    let (winner, cards) = if one_wins {(deck1, [card1, card2])} else {(deck2, [card2, card1])};
    cards.iter().for_each(|c| winner.cards.push_back(*c));
}

fn play_puzzle_1_to_end(deck1: &mut Deck, deck2: &mut Deck) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        play_puzzle_1_one_round(deck1, deck2);
    }
    score(deck2.is_empty(), deck1, deck2)
}

fn score(one_wins: bool, deck1: &Deck, deck2: &Deck) -> usize {
    (if one_wins {deck1} else {deck2}).score()
}

struct Puzzle2Game {
    previous_rounds: BTreeSet<(Deck,Deck)>    
}

impl Puzzle2Game {
    fn new() -> Self {
        Puzzle2Game { previous_rounds: BTreeSet::new() }
    }

    fn player_1_wins(&mut self, deck1: &mut Deck, deck2: &mut Deck) -> (bool,usize) {
        while !deck1.is_empty() && !deck2.is_empty() {
            let index = (deck1.clone(), deck2.clone());
            if self.previous_rounds.contains(&index) {
                return (true, deck1.score())
            }
            self.play_one_round(deck1, deck2);
            self.previous_rounds.insert(index);
        }
        let one_wins = deck2.is_empty();
        (one_wins, score(one_wins, deck1, deck2))
    }
    
    fn play_one_round(&mut self, deck1: &mut Deck, deck2: &mut Deck) {
        let card1 = deck1.cards.pop_front().unwrap();
        let card2 = deck2.cards.pop_front().unwrap();
        let one_wins = if card1 <= deck1.len() && card2 <= deck2.len() {
            let mut recurred = Puzzle2Game::new();
            let (one_wins, _) = recurred.player_1_wins(&mut deck1.copy_n(card1), &mut deck2.copy_n(card2));
            one_wins
        } else {
            card1 > card2
        };
        resolve_winner(one_wins, card1, card2, deck1, deck2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(solve_1("in/day22_ex.txt").unwrap(), "306");
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(solve_2("in/day22_ex.txt").unwrap(), "291");
    }
}