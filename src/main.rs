mod cards;

use cards::{Card, Rank, Shoe};

fn main() {
    println!("yeah buddy");

    let mut shoe = Shoe::new(4); //example 4 deck shoe
                                 // shuffle the shoe
    shoe.shuffle();

    //instantiate players
    let mut player = Player::new();
    let mut dealer = Player::new();

    // deal initial cards
    player.add_card(shoe.deal().unwrap());
    dealer.add_card(shoe.deal().unwrap());
    player.add_card(shoe.deal().unwrap());
    dealer.add_card(shoe.deal().unwrap());

    println!("Hand: {:?}", player.hand);
    println!("Score: {:?}", player.score());
    println!("Dealer: {:?}", dealer.hand[0]);
}

struct Player {
    hand: Vec<Card>,
}

impl Player {
    fn new() -> Self {
        Player { hand: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        let mut aces = 0;
        for card in &self.hand {
            match card.rank {
                Rank::Ace => {
                    score += 10;
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
