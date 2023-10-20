use crate::cards::{Card, Hand, Rank};

pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Hand::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.cards.push(card);
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut aces = 0;
        for card in &self.hand.cards {
            match card.rank {
                Rank::Ace => {
                    score += 11;
                    aces += 1;
                }
                Rank::Two => {
                    score += 2;
                }
                Rank::Three => {
                    score += 3;
                }
                Rank::Four => {
                    score += 4;
                }
                Rank::Five => {
                    score += 5;
                }
                Rank::Six => {
                    score += 6;
                }
                Rank::Seven => {
                    score += 7;
                }
                Rank::Eight => {
                    score += 8;
                }
                Rank::Nine => {
                    score += 9;
                }
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => {
                    score += 10;
                }
            }
        }

        // if score > 21 and you have an ace, decrement score by 10
        while score > 21 && aces > 0 {
            score -= 10;
            aces -= 1;
        }
        score
    }
}
