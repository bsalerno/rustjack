use crate::cards::{Hand, Shoe};
use std::io;

pub struct Player {
    pub hands: Vec<Hand>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hands: vec![Hand::new()],
        }
    }
}

fn settle_player_hand<'a>(shoe: &'a mut Shoe, player: &'a mut Hand) -> f32 {
    //(&mut Hand, f32)
    // set initial hand multiplier to 1
    // this will increase to 2 if doubled down
    let mut multiplier: f32 = 1.0;
    // player's turn
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
                println!("Hand: {}", player);
                println!("Score: {:?}", player.score());
                if player.score() > 21 {
                    break;
                }
                counter += 1;
            }
            ("d", 1) => {
                multiplier = 2.0;
                player.add_card(shoe.deal().unwrap());
                println!("Hand: {}", player);
                println!("Score: {:?}", player.score());
                if player.score() > 21 {
                    break;
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

    // return player hand and hand multlplier
    // (player, multiplier)
    // player
    multiplier
}

fn settle_dealer_hand<'a>(shoe: &'a mut Shoe, dealer: &'a mut Hand) -> () {
    // dealer's turn
    println!("Dealer: {}", dealer);
    while dealer.score() < 17 {
        dealer.add_card(shoe.deal().unwrap());
        println!("Dealer: {}", dealer);
    }
    println!("Dealer score: {}", dealer.score());

    // dealer
}

pub fn play_hand(shoe: &mut Shoe) -> f32 {
    //instantiate players
    let mut player = Player::new();
    let mut dealer = Player::new();

    // deal initial cards
    player.hands[0].add_card(shoe.deal().unwrap());
    dealer.hands[0].add_card(shoe.deal().unwrap());
    player.hands[0].add_card(shoe.deal().unwrap());
    dealer.hands[0].add_card(shoe.deal().unwrap());

    println!("Hand: {}", player.hands[0]);
    println!("Score: {:?}", player.hands[0].score());

    // print dealer's up card
    println!("Dealer: {}", dealer.hands[0].cards[0]);

    // handle blackjack
    if player.hands[0].score() == 21 && dealer.hands[0].score() == 21 {
        println!("Dealer: {}", dealer.hands[0]);
        println!("Pushed blackjack!");
        return 0.0;
    } else if player.hands[0].score() == 21 {
        println!("Blackjack!");
        return 1.5;
    }

    //allow splitting
    if player.hands[0].cards[0].rank == player.hands[0].cards[1].rank {
        println!("Split pair? [y/n]");
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "n" => {}
                "y" => {
                    player.hands.push(Hand { cards: vec![] });
                    player.hands[1].add_card(player.hands[0].cards[1]);
                    player.hands[0].add_card(shoe.deal().unwrap());
                    player.hands[1].add_card(shoe.deal().unwrap());
                }
                _ => {
                    println!("Invalid choice, yes (y) or no (n).")
                }
            }
        }
    }

    let multiplier = settle_player_hand(shoe, &mut player.hands[0]);

    if player.hands[0].score() > 21 {
        println!("Busted!");
        println!("Dealer wins!");
        return -1.0;
    }

    settle_dealer_hand(shoe, &mut dealer.hands[0]);

    if dealer.hands[0].score() > 21 || player.hands[0].score() > dealer.hands[0].score() {
        println!("Player wins!");
        return 1.0 * multiplier;
    } else if dealer.hands[0].score() > player.hands[0].score() {
        println!("Dealer wins!");
        return -1.0 * multiplier;
    } else {
        println!("Push!");
        return 0.0;
    }
}
