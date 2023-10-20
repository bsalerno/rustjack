use crate::cards::{Card, Hand, Rank, Shoe};
use std::io;

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

pub fn play_hand(shoe: &mut Shoe) -> f32 {
    //instantiate players
    let mut player = Player::new();
    let mut dealer = Player::new();

    // set initial hand multiplier to 1
    // this will increase to 2 if doubled down
    let mut multiplier: f32 = 1.0;
    // deal initial cards
    player.add_card(shoe.deal().unwrap());
    dealer.add_card(shoe.deal().unwrap());
    player.add_card(shoe.deal().unwrap());
    dealer.add_card(shoe.deal().unwrap());

    println!("Hand: {}", player.hand);
    println!("Score: {:?}", player.score());

    if player.score() == 21 {
        println!("Blackjack!");
        return 1.5;
    }

    // print dealer's up card
    println!("Dealer: {}", dealer.hand.cards[0]);

    //player's turn
    let mut counter: u32 = 1;
    loop {
        match counter {
            1 => {
                println!("hit (h), stand (s), or double down (d)?");
            }
            _ => {
                println!("hit (h) or stand (s)?");
            }
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // construct a tuple for instructions and counter
        let decision = (input.trim(), counter);

        match decision {
            ("h", _) => {
                player.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player.hand);
                println!("Score: {:?}", player.score());
                if player.score() > 21 {
                    println!("Busted.");
                    return -1.0;
                }
                counter += 1;
            }
            ("d", 1) => {
                multiplier = 2.0;
                player.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player.hand);
                println!("Score: {:?}", player.score());
                if player.score() > 21 {
                    println!("Busted.");
                    return -2.0;
                }
                // you can only get one card when doubling down
                break;
            }
            ("s", _) => {
                break;
            }
            (_, 1) => {
                println!("Invalid choice, hit (h), stand (s), or double down (d).");
            }
            (_, _) => {
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
    println!("Dealer score: {}", dealer.score());

    if dealer.score() > 21 || player.score() > dealer.score() {
        println!("Player wins!");
        return 1.0 * multiplier;
    } else if dealer.score() > player.score() {
        println!("Dealer wins!");
        return -1.0 * multiplier;
    } else {
        println!("Push!");
        return 0.0;
    }
}
