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
    let part = args.part;
    let input = std::fs::read_to_string(&file).unwrap();

    if part == 0 {
        let solution_one: i32 = part_one(input.clone());
        println!("{solution_one}");
    }
    if part == 1 {
        let solution_two = part_two(input.clone());
        println!("{solution_two}");
    }
}

fn part_one(input: String) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
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

    return sum;
}

fn part_two(input: String) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut numbers = Vec::new();
        let line = &line;
        for i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            let rest: String = line.chars().skip(i).take(line.len() - i).collect();
            if rest.starts_with("one") {
                numbers.push(1);
            }
            if rest.starts_with("two") {
                numbers.push(2);
            }
            if rest.starts_with("three") {
                numbers.push(3);
            }
            if rest.starts_with("four") {
                numbers.push(4);
            }
            if rest.starts_with("five") {
                numbers.push(5);
            }
            if rest.starts_with("six") {
                numbers.push(6);
            }
            if rest.starts_with("seven") {
                numbers.push(7);
            }
            if rest.starts_with("eight") {
                numbers.push(8);
            }
            if rest.starts_with("nine") {
                numbers.push(9);
            }

            if char.is_numeric() {
                numbers.push(char.to_string().parse::<i32>().unwrap())
            }
        }

        if numbers.len() == 0 {
            numbers.push(0);
        }

        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        let num = 10 * first + last;
        sum += num;
    }

    return sum;
}
