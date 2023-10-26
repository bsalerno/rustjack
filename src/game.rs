use crate::cards::{Hand, Rank, Shoe};
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

    pub fn split(&mut self) -> () {
        let new_hand = Hand {
            cards: vec![self.hands[0].cards.remove(1)],
            multiplier: 1.0,
        };

        self.hands.push(new_hand);
    }
}

fn settle_player_hand<'a>(shoe: &'a mut Shoe, player: &'a mut Hand, dealer: &'a mut Hand) -> () {
    println!("Hand: {}", player);
    println!("Score: {:?}", player.score());

    // print dealer's up card
    println!("Dealer: {}", dealer.cards[0]);
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
                player.multiplier = 2.0;
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

    // handle blackjack
    if player.hands[0].score() == 21 && dealer.hands[0].score() == 21 {
        println!("Hand: {}", player.hands[0]);
        println!("Score: {:?}", player.hands[0].score());

        println!("Dealer: {}", dealer.hands[0]);
        println!("Pushed blackjack!");
        return 0.0;
    } else if player.hands[0].score() == 21 {
        println!("Hand: {}", player.hands[0]);
        println!("Score: {:?}", player.hands[0].score());

        // print dealer's up card
        println!("Dealer: {}", dealer.hands[0].cards[0]);

        println!("Blackjack!");
        return 1.5;
    }

    // offer insurance
    let mut insurance: bool = false;
    if dealer.hands[0].cards[0].rank == Rank::Ace {
        println!("Hand: {}", player.hands[0]);
        println!("Score: {:?}", player.hands[0].score());

        // print dealer's up card
        println!("Dealer: {}", dealer.hands[0].cards[0]);
        println!("Insurance? [y/n]");
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "n" => {
                    break;
                }
                "y" => {
                    insurance = true;
                    break;
                }
                _ => {
                    println!("Invalid choice, yes (y) or no (n).")
                }
            }
        }
    }

    //allow splitting
    if player.hands[0].can_split() {
        println!("Hand: {}", player.hands[0]);
        println!("Score: {:?}", player.hands[0].score());

        // print dealer's up card
        println!("Dealer: {}", dealer.hands[0].cards[0]);

        println!("Split pair? [y/n]");
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "n" => {
                    break;
                }
                "y" => {
                    player.split();
                    player.hands[0].add_card(shoe.deal().unwrap());
                    player.hands[1].add_card(shoe.deal().unwrap());
                    break;
                }
                _ => {
                    println!("Invalid choice, yes (y) or no (n).")
                }
            }
        }
    }

    for h in player.hands.iter_mut() {
        settle_player_hand(shoe, h, &mut dealer.hands[0]);
    }

    settle_dealer_hand(shoe, &mut dealer.hands[0]);

    let mut winnings: f32 = 0.0;
    for h in player.hands {
        if h.score() > 21 {
            println!("Busted!");
            println!("Dealer wins!");
            winnings += -1.0 * h.multiplier;
        } else if dealer.hands[0].score() > 21 || h.score() > dealer.hands[0].score() {
            println!("Player wins!");
            winnings += 1.0 * h.multiplier;
        } else if dealer.hands[0].score() > h.score() {
            println!("Dealer wins!");
            winnings += -1.0 * h.multiplier;
        } else {
            println!("Push!");
            winnings += 0.0;
        }
    }

    if insurance
        && matches!(
            dealer.hands[0].cards[1].rank,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King
        )
    {
        println!("Insurance pays!");
        winnings += 1.0;
    } else if insurance {
        winnings -= 1.0;
    }

    //return total winnings
    winnings
}
