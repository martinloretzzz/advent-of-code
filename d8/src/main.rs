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

    for line in lines {
        let line_parts = line.split(" = ").collect::<Vec<&str>>();
        let [from, dest] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        let line_parts = dest.split(", ").collect::<Vec<&str>>();
        let [dest_left, dest_right] = <[&str; 2]>::try_from(line_parts).ok().unwrap();

        network_left.insert(from, dest_left);
        network_right.insert(from, dest_right);

        // println!("s={from} left={dest_left} right={dest_right}");
    }

    let mut step_counter = 0;
    let mut pos = "AAA";
    loop {
        let next_step = directions[step_counter % directions.len()];
        if next_step == "L" {
            pos = network_left[pos];
        } else {
            pos = network_right[pos];
        };
        step_counter += 1;
        // println!("{next_step}=>{pos}");
        if pos == "ZZZ" {
            break;
        }
    }

    println!("Steps: {step_counter}")
}
