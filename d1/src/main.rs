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
    let mut sum = 0;

    for line in std::fs::read_to_string(&file).unwrap().lines() {
        let mut numbers = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char.to_string().parse::<i32>().unwrap())
            }
        }
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        let num = 10 * first + last;
        sum += num;
    }

    println!("{sum}");
}
