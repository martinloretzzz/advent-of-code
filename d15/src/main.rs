use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

fn holiday_hash(input: &str) -> i32 {
    let mut hash: i32 = 0;
    for char in input.chars() {
        let ascii = char as i32;
        hash = ((hash + ascii) * 17) % 256;
    }
    return hash;
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let mut sum = 0;
    let parts: Vec<&str> = input.split(",").filter(|s| !s.is_empty()).collect();

    for part in parts {
        let hash = holiday_hash(part);
        sum += hash;
        println!("{hash}");
    }

    println!("Sum of hashes: {sum}");
}
