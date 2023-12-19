use clap::Parser;
use memoize::memoize;
use std::{cmp::Ordering, collections::HashMap, path::PathBuf};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

#[memoize]
fn get_card_type(cards: String) -> i32 {
    let cards = cards
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut card_counts = HashMap::new();

    for card in cards {
        if let Some(x) = card_counts.get_mut(card) {
            *x += 1;
        } else {
            card_counts.insert(card, 1);
        }
    }

    let (mut five_count, mut four_count, mut triple_count, mut pair_count) = (0, 0, 0, 0);

    for count in card_counts.into_values() {
        if count == 5 {
            five_count += 1;
        }
        if count == 4 {
            four_count += 1;
        }
        if count == 3 {
            triple_count += 1;
        }
        if count == 2 {
            pair_count += 1;
        }
    }

    if five_count == 1 {
        // Five of a kind
        return 7;
    } else if four_count == 1 {
        // Four of a kind
        return 6;
    } else if triple_count == 1 && pair_count == 1 {
        // Full house
        return 5;
    } else if triple_count == 1 {
        // Three of a kind
        return 4;
    } else if pair_count == 2 {
        // Two pair
        return 3;
    } else if pair_count == 1 {
        // One pair
        return 2;
    } else {
        // High card
        return 1;
    }
}

fn card_to_number(card: &str) -> u32 {
    return match card {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        _ => card
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            .to_digit(10)
            .unwrap(),
    };
}

fn sort_game_by_higher_card(a: String, b: String) -> Ordering {
    let a_cards = a.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    let b_cards = b.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>();

    for i in 0..a_cards.len() {
        let a_number = card_to_number(a_cards[i]);
        let b_number = card_to_number(b_cards[i]);

        if a_number != b_number && a_number < b_number {
            return Ordering::Less;
        } else if a_number != b_number && a_number > b_number {
            return Ordering::Greater;
        }
    }
    panic!();
}

fn sort_cards(a: String, b: String) -> Ordering {
    let a_type = get_card_type(a.clone());
    let b_type = get_card_type(b.clone());

    if a_type == b_type {
        return sort_game_by_higher_card(a, b);
    } else if a_type < b_type {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let mut games: Vec<String> = Vec::new();
    let mut game_bet_amount = HashMap::new();

    for line in input.lines() {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let [game, amount] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        games.push(game.to_string());
        game_bet_amount.insert(game.to_string(), amount.parse::<i32>().unwrap());
    }

    games.sort_by(|a, b| sort_cards(a.to_string(), b.to_string()));

    let mut winnings = 0;
    for (i, game) in games.into_iter().enumerate() {
        let rank = (i + 1) as i32;
        let amount = game_bet_amount[&game];
        winnings += rank * amount;
        println!("{rank} {game} {amount}");
    }

    println!("Total winnings: {winnings}");
}
