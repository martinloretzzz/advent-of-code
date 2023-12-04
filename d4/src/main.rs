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

    let mut sum = 0;

    for line in input.lines() {
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
        for guess in guess_numbers {
            if winning_numbers.contains(&guess) {
                card_points = if card_points == 0 { 1 } else { 2 * card_points };
            }
        }
        sum += card_points;
    }

    println!("Card points: {sum}");
}
