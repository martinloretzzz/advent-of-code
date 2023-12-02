use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
    part: i32,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split(':');

        let game_id = split.next().unwrap();
        let game_data = split.next().unwrap();

        let mut game_id_parts = game_id.split(" ");
        game_id_parts.next();
        let game_id_num = game_id_parts.next().unwrap().parse::<i32>().unwrap();

        let mut invalid = false;

        for take in game_data.split(";") {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            for cube in take.split(",") {
                let cube = cube.trim();
                let mut cube_parts = cube.split(" ");
                let count = cube_parts.next().unwrap().parse::<i32>().unwrap();
                let color = cube_parts.next().unwrap();

                if color == "red" {
                    red_count += count;
                }
                if color == "green" {
                    green_count += count;
                }
                if color == "blue" {
                    blue_count += count;
                }
            }
            if red_count > 12 || green_count > 13 || blue_count > 14 {
                invalid = true;
            }
        }
        if !invalid {
            sum += game_id_num;
        }
    }
    println!("{sum}");
}
