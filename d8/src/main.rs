use clap::Parser;
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file)
        .unwrap()
        .replace("(", "")
        .replace(")", "");

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let directions: Vec<&str> = first_line.split("").filter(|s| !s.is_empty()).collect();

    lines.next();

    let mut network_left = HashMap::new();
    let mut network_right = HashMap::new();

    let mut start_positions: Vec<&str> = Vec::new();

    for line in lines {
        let line_parts = line.split(" = ").collect::<Vec<&str>>();
        let [from, dest] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        let line_parts = dest.split(", ").collect::<Vec<&str>>();
        let [dest_left, dest_right] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        network_left.insert(from, dest_left);
        network_right.insert(from, dest_right);

        if from.ends_with("A") {
            start_positions.push(from);
        }

        // println!("s={from} left={dest_left} right={dest_right}");
    }

    let mut step_counter = 0;
    let mut gost_positions = start_positions.clone();
    let ghost_count = start_positions.len();
    loop {
        let next_step = directions[step_counter % directions.len()];
        let mut all_finished = true;
        for i in 0..ghost_count {
            let mut pos = gost_positions[i];
            if next_step == "L" {
                pos = network_left[pos];
            } else {
                pos = network_right[pos];
            }
            if !pos.ends_with("Z") {
                all_finished = false;
            } else {
                println!("{i}: {step_counter}");
            }
            gost_positions[i] = pos;
        }
        step_counter += 1;
        if all_finished {
            break;
        }
    }

    println!("Steps: {step_counter}")
}

// Solved by finding the smallest common multiply of the positions where the ghosts land on Z:
// Steps between Z positions: 20093	12169	22357	14999	13301	17263
// Smallest common multiply: 10371555451871
