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
    let mut sum_power_cube_min = 0;

    for line in input.lines() {
        let line_parts = line.split(':').collect::<Vec<&str>>();
        let [game_id, game_data] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        let game_id_parts = game_id.split(" ").collect::<Vec<&str>>();
        let game_id_num = game_id_parts[1].parse::<i32>().unwrap();

        let mut invalid = false;

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        for take in game_data.split(";") {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            for cube in take.split(",") {
                let cube = cube.trim();
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let count = cube_parts[0].parse::<i32>().unwrap();
                let color = cube_parts[1];

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
            red_min = red_min.max(red_count);
            green_min = green_min.max(green_count);
            blue_min = blue_min.max(blue_count);
        }
        if !invalid {
            sum += game_id_num;
        }
        let min_power = red_min * green_min * blue_min;
        sum_power_cube_min += min_power;
    }

    println!("Sum of Ids: {sum}");
    println!("Sum of the powers of the min cubes: {sum_power_cube_min}");
}
