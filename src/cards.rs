use rand::prelude::*;
use rand::thread_rng;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let p = match *self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };

        write!(f, "{}", p)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let p = match *self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };

        write!(f, "{}", p)
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in self.cards.iter() {
            write!(f, "{} ", item)?
        }
        Ok(())
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        // iterate through suits and ranks to create deck
        for &s in &suits {
            for &r in &ranks {
                cards.push(Card { suit: s, rank: r });
            }
        }

        Deck { cards: cards }
    }
}

pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(deck_count: i8) -> Self {
        let mut cards = Vec::new();

        for _ in 0..deck_count {
            cards.extend(Deck::new().cards);
        }

        Shoe { cards: cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_has_52_cards() {
        assert_eq!(Deck::new().cards.len(), 52);
    }
}
