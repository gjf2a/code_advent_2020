use std::collections::VecDeque;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (mut deck1, mut deck2) = decks_from(filename)?;
    Ok(play_to_end(&mut deck1, &mut deck2).to_string())
}

fn decks_from(filename: &str) -> io::Result<(Deck,Deck)> {
    let mut iter = all_lines(filename)?;
    let deck1 = Deck::from(&mut iter);
    let deck2 = Deck::from(&mut iter);
    Ok((deck1, deck2))
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Deck {
    player: String,
    cards: VecDeque<u64>
}

impl Deck {
    fn from<I:Iterator<Item=String>>(iter: &mut I) -> Self {
        let player = iter.next().unwrap();
        let cards = iter
            .take_while(|s| s.len() > 0)
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Deck {player, cards}
    }

    fn empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn score(&self) -> u64 {
        self.cards.iter().rev().enumerate()
            .map(|(count, value)| (count + 1) as u64 * value)
            .sum()
    }
}

fn play_one_round(deck1: &mut Deck, deck2: &mut Deck) {
    let card1 = deck1.cards.pop_front().unwrap();
    let card2 = deck2.cards.pop_front().unwrap();
    let (winner, cards) = if card1 > card2 {(deck1, [card1, card2])} else {(deck2, [card2, card1])};
    cards.iter().for_each(|c| winner.cards.push_back(*c));
}

fn play_to_end(deck1: &mut Deck, deck2: &mut Deck) -> u64 {
    while !deck1.empty() && !deck2.empty() {
        play_one_round(deck1, deck2);
    }
    (if deck1.empty() {deck2} else {deck1}).score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(solve_1("in/day22_ex.txt").unwrap(), "306");
    }
}