mod cards;

use cards::{Card, Hand, Rank, Shoe};
use std::io;

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

    println!("Hand: {}", player.hand);
    println!("Score: {:?}", player.score());
    println!("Dealer: {}", dealer.hand.cards[0]);

    //player's turn
    loop {
        println!("hit (h) or stand (s)?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "h" => {
                player.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player.hand);
                println!("Score: {:?}", player.score());
                if player.score() > 21 {
                    println!("Busted.");
                    return;
                }
            }
            "s" => {
                break;
            }
            _ => {
                println!("Invalid choice, hit (h) or stand (s).");
            }
        }
    }

    // dealer's turn
    println!("Dealer: {}", dealer.hand);
    while dealer.score() < 17 {
        dealer.add_card(shoe.deal().unwrap());
        println!("Dealer: {}", dealer.hand);
    }

    if dealer.score() > 21 || player.score() > dealer.score() {
        println!("Player wins!");
    } else if dealer.score() > player.score() {
        println!("Dealer wins!");
    } else {
        println!("Push!");
    }
}

struct Player {
    hand: Hand,
}

impl Player {
    fn new() -> Self {
        Player { hand: Hand::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.hand.cards.push(card);
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        let mut aces = 0;
        for card in &self.hand.cards {
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
