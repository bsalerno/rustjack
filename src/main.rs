mod cards;
mod game;

use cards::Shoe;
use game::play_hand;

fn main() {
    println!("yeah buddy");

    let mut plusminus: f32 = 0.0;
    let mut shoe = Shoe::new(4); //example 4 deck shoe

    // shuffle the shoe
    shoe.shuffle();

    plusminus += play_hand(shoe);
    println!("Current +/-: {} units", plusminus);
}
