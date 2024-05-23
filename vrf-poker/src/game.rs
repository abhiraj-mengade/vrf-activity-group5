extern crate schnorrkel;
use crate::{recieve, try_draw};
use rand::Rng;
use schnorrkel::{signing_context, Keypair, PublicKey};
use sha2::{Digest, Sha256};
use std::io::stdin;

#[derive(Debug)]
struct Player {
    Keypair: Keypair,
    cards: Vec<(u16, [u8; 97])>,
    balance: i32,
}

impl Player {
    pub fn new(keypair: Keypair, balance: i32) -> Player {
        Player {
            Keypair: keypair,
            cards: Vec![],
            balance,
        }
    }
    pub fn draw_hand(&mut self, cards: Vec<(u16, [u8; 97])>) {
        self.cards = cards;
    }
}

pub fn run() {
    println!("Welcome to VRF Poker!");
    printnln!("How many players are there? ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let num_players: i32 = input.trim().parse().unwrap();
    println!(
        "All {} players have joined the game! Each one of you gets a 1000SGD balance. Let's start!",
        num_players
    );

    let cspring = rand_core::OsRng;
    let mut players: Vec<Player> = (0..num_players)
        .map(|_| {
            let keypair = Keypair::generate_with(&mut cspring);
            Player::new(keypair, 1000)
        })
        .collect();

    // Signing
    let context = signing_context(b"VRF Poker");
    let message = b"I am a player in VRF Poker!";
    let signs = players.iter().fold(Vec::new(), |mut acc, player| {
        let mut sig = player
            .Keypair
            .sign(context.bytes(message))
            .to_bytes()
            .to_vec();

        acc.push(&mut sig);
        acc
    });

    //Creating a VRF seed.
    let mut hasher = Sha256::new();
    hasher.update(signs);
    let hash = hasher.finalize();
    let vrf_seed = hash.as_slice().try_into().unwrap();

    // Drawing cards
    players.iter_mut().for_each(|player| {
        let cards = (0..2)
            .filter_map(|_| try_draw(&player.Keypair, &vrf_seed, i))
            .collect();
        player.draw_hand(cards);
    });

    // Recieving cards
    let mut bank = 0;
    println!("Everyone has been given 2 cards!");
    wait();

    bid(&mut players, &mut bank);

    println!("Bank has {}SGD", bank);
    wait();

    let table_key = Keypair::generate_with(&mut cspring);
    let mut cards = (0..3)
        .filter_map(|_| try_draw(&table_key, &vrf_seed, i))
        .collect();

    println!(
        "The table has been dealt 3 cards! {:?}",
        reveal_cards(&cards, &table.public, vrf_seed)
    );
    wait();
    bid(&mut players, &mut bank);
    wait();

    //Draw 4th Card
    let card = try_draw(&table_key, &vrf_seed, 3).unwrap();
    cards.push(card);

    println!(
        "The table has been dealt 4 cards! {:?}",
        reveal_cards(&cards, &table.public, vrf_seed)
    );

    wait();
    bid(&mut players, &mut bank);
    wait();

    //Draw 5th Card
    let card = try_draw(&table_key, &vrf_seed, 4).unwrap();
    cards.push(card);

    println!(
        "The table has been dealt 5 cards! {:?}",
        reveal_cards(&cards, &table.public, vrf_seed)
    );

    let cards_revealed = reveal_cards(&cards, &table.public, vrf_seed);

    wait();
    bid(&mut players, &mut bank);
    wait();

    //Determining the winner
    let table_sum = cards_revealed.iter().sum();
    let mut highest_score = (0, &PublicKey::default());
    players.iter().for_each(|player| {
        let player_cards = reveal_cards(&player.cards, &player.Keypair.public, vrf_seed);
        println!(
            "Player {:?} has cards {:?}",
            player.Keypair.public, player_cards
        );
        let player_sum = player_cards.iter().sum() + table_sum;
        if player_sum > highest_score.0 {
            highest_score = (player_sum, &player.Keypair.public);
        }
    });

    println!(
        "The winner is {:?}",
        highest_score.1,
        highest_score.1.to_bytes(),
        highest_score,
        bank
    );
}

fn wait() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}

fn bid(players: &mut Vec<Player>, bank: &mut i32) {
    players.iter_mut().for_each(|player| {
        let bid = rand::thread_rng().gen_range(0..302);
        player.balance -= bid;
        println!(
            "Player {:?} has bid {}SGD. They have {}SGD left.",
            player.Keypair.public, bid, player.balance
        );
        *bank += bid;
    });
}

fn reveal_cards(cards: &Vec<(u16, [u8; 97])>, public: &PublicKey, vrf_seed: &[u8; 32]) -> Vec<u16> {
    cards
        .iter()
        .filter_map(|card| recieve(public, &card, vrf_seed))
        .collect();
}
