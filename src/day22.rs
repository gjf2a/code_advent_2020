use std::collections::{VecDeque, BTreeSet};
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    solve(filename, false)
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    solve(filename, true)
}

fn solve(filename: &str, recursive: bool) -> io::Result<String> {
    let mut game = Game::from(filename, recursive)?;
    let (_, score) = game.play_to_end();
    Ok(score.to_string())
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
enum Player {One, Two}

impl Player {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Player 1:" => Player::One,
            "Player 2:" => Player::Two,
            _ => panic!("Shouldn't happen")
        }
    }
}

struct Game {
    deck1: Deck, deck2: Deck, recursive: bool, previous_rounds: BTreeSet<(Deck,Deck)>
}

impl Game {
    fn from(filename: &str, recursive: bool) -> io::Result<Self> {
        let mut iter = all_lines(filename)?;
        let deck1 = Deck::from(&mut iter);
        let deck2 = Deck::from(&mut iter);
        Ok(Game { deck1, deck2, recursive, previous_rounds: BTreeSet::new() })
    }

    fn subgame(&self, card1: usize, card2: usize) -> Self {
        Game { deck1: self.deck1.copy_n(card1), deck2: self.deck2.copy_n(card2),
            recursive: self.recursive, previous_rounds: BTreeSet::new() }
    }

    fn final_score(&self) -> Option<(Player,usize)> {
        if self.deck1.is_empty() {
            Some(self.deck2.score())
        } else if self.deck2.is_empty() {
            Some(self.deck1.score())
        } else {
            None
        }
    }

    fn play_to_end(&mut self) -> (Player,usize) {
        loop {
            if self.repeated() {
                return self.deck1.score();
            }
            self.play_one_round();
            if let Some(outcome) = self.final_score() {
                return outcome;
            }
        }
    }

    fn repeated(&mut self) -> bool {
        self.recursive && {
            let key = (self.deck1.clone(), self.deck2.clone());
            let outcome = self.previous_rounds.contains(&key);
            self.previous_rounds.insert(key);
            outcome
        }
    }

    fn play_one_round(&mut self) {
        let card1 = self.deck1.draw();
        let card2 = self.deck2.draw();
        let winner = self.winner(card1, card2);
        self.insert_cards(winner, card1, card2);
    }

    fn winner(&self, card1: usize, card2: usize) -> Player {
        if self.recursive && card1 <= self.deck1.len() && card2 <= self.deck2.len() {
            let mut recurred = self.subgame(card1, card2);
            let (winner, _) = recurred.play_to_end();
            winner
        } else {
            if card1 > card2 {Player::One} else {Player::Two}
        }
    }

    fn insert_cards(&mut self, winner: Player, card1: usize, card2: usize) {
        let (winner, cards) = if winner == Player::One {
            (&mut self.deck1, [card1, card2])
        } else {
            (&mut self.deck2, [card2, card1])
        };
        cards.iter().for_each(|c| winner.add(*c));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Deck {
    player: Player,
    cards: VecDeque<usize>
}

impl Deck {
    fn from<I:Iterator<Item=String>>(iter: &mut I) -> Self {
        let player = Player::from(iter.next().unwrap());
        let cards = iter
            .take_while(|s| s.len() > 0)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Deck {player, cards}
    }

    fn draw(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    fn add(&mut self, card: usize) {
        self.cards.push_back(card);
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

    fn score(&self) -> (Player,usize) {
        (self.player,
         self.cards.iter().rev().enumerate()
            .map(|(count, value)| (count + 1) * value)
            .sum())
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