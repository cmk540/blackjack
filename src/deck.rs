use crate::card::{Card, Rank};
use rand::{seq::SliceRandom, thread_rng};

pub const SINGLE_DECK_SIZE: usize = 52;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Deck {
    stack: Vec<Card>,
}

impl Deck {
    pub fn new_shoe(decks: usize) -> Self {
        let stack: Vec<Card> = (0..52)
            .cycle()
            .take(SINGLE_DECK_SIZE * decks)
            .map(|c: u8| c.try_into().unwrap() )
            .collect();

        Self { stack }
    }

    pub fn stack(&self) -> Vec<Card> {
        self.stack.clone()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.stack.pop()
    }

    pub fn cards_left(&self) -> usize {
        self.stack.len()
    }

    pub fn mega_true_count(&self) -> i64 {
        let mut count: i64 = 0;

        for c in &self.stack {
            match c.rank() {
                Rank::Ace => count += -1,
                Rank::Two => count += 1,
                Rank::Three => count += 1,
                Rank::Four => count += 1,
                Rank::Five => count += 1,
                Rank::Six => count += 1,
                Rank::Ten => count += -1,
                Rank::Jack => count += -1,
                Rank::Queen => count += -1,
                Rank::King => count += -1,
                _ => {},
            }
        }

        count
    }

    pub fn shuffle(&mut self) {
        self.stack.shuffle(&mut thread_rng());
    }
}