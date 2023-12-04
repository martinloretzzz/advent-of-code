use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let input_card_count = input.lines().count();

    let mut sum = 0;
    let mut count = 0;
    let mut cards = vec![0; input_card_count];

    for (card_id, line) in input.lines().enumerate() {
        let line_parts = line.split(':').collect::<Vec<&str>>();
        let [_game_id, game_data] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        let game_parts = game_data.split("|").collect::<Vec<&str>>();
        let [winning_numbers, guess_numbers] = <[&str; 2]>::try_from(game_parts).ok().unwrap();

        let winning_numbers: Vec<i32> = winning_numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        let guess_numbers: Vec<i32> = guess_numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut card_points = 0;
        let mut winning_count = 0;
        for guess in guess_numbers {
            if winning_numbers.contains(&guess) {
                card_points = if card_points == 0 { 1 } else { 2 * card_points };
                winning_count += 1;
            }
        }
        sum += card_points;

        let num_of_cards = cards[card_id] + 1;
        for offset in 1..winning_count + 1 {
            cards[card_id + offset] += num_of_cards;
            count += num_of_cards;
        }
    }

    count += input_card_count;

    println!("Card points: {sum}");
    println!("Card count: {count}");
}
