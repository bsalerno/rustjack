mod cards;
mod game;

use cards::Shoe;
use clap::Parser;
use game::play_hand;

fn main() {
    println!("yeah buddy");
    let args = Args::parse();

    let mut plusminus: f32 = 0.0;
    let mut shoe = Shoe::new(args.decks); //example 4 deck shoe

    // shuffle the shoe
    shoe.shuffle();

    loop {
        plusminus += play_hand(&mut shoe);
        println!("Current +/-: {} units", plusminus);
        if shoe.should_reshuffle(0.3) {
            shoe = Shoe::new(args.decks);
            shoe.shuffle();
            println!("Reshuffling...");
        }
        println!("--------------------------------------");
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// number of decks in the shoe to play with
    #[arg(short, long, default_value_t = 8)]
    decks: u8,
}
