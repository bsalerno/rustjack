mod cards;
mod game;

use cards::Shoe;
use game::Player;
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

    if player.score() == 21 {
        println!("Blackjack!");
        return;
    }

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
