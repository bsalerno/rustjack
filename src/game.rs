use crate::cards::{Hand, Shoe};
use std::io;

pub struct Player {
    pub hand: Hand,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Hand::new() }
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
    player.hand.add_card(shoe.deal().unwrap());
    dealer.hand.add_card(shoe.deal().unwrap());
    player.hand.add_card(shoe.deal().unwrap());
    dealer.hand.add_card(shoe.deal().unwrap());

    println!("Hand: {}", player.hand);
    println!("Score: {:?}", player.hand.score());

    if player.hand.score() == 21 {
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
                player.hand.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player.hand);
                println!("Score: {:?}", player.hand.score());
                if player.hand.score() > 21 {
                    println!("Busted.");
                    return -1.0;
                }
                counter += 1;
            }
            ("d", 1) => {
                multiplier = 2.0;
                player.hand.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player.hand);
                println!("Score: {:?}", player.hand.score());
                if player.hand.score() > 21 {
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
    while dealer.hand.score() < 17 {
        dealer.hand.add_card(shoe.deal().unwrap());
        println!("Dealer: {}", dealer.hand);
    }
    println!("Dealer score: {}", dealer.hand.score());

    if dealer.hand.score() > 21 || player.hand.score() > dealer.hand.score() {
        println!("Player wins!");
        return 1.0 * multiplier;
    } else if dealer.hand.score() > player.hand.score() {
        println!("Dealer wins!");
        return -1.0 * multiplier;
    } else {
        println!("Push!");
        return 0.0;
    }
}
