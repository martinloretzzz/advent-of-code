use clap::Parser;
use core::panic;
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

    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let size = input.lines().collect::<Vec<&str>>().len();
    for (y, line) in input.lines().enumerate() {
        let mut num: u32 = 0;
        let mut num_start: i32 = -1;
        let mut num_end: i32 = -1;

        let zero_row = vec![0; size];
        numbers.push(zero_row);

        for (x, pos) in line.chars().enumerate() {
            let x_size = line.chars().collect::<Vec<char>>().len();
            if pos.is_ascii_digit() {
                if num_start == -1 {
                    num_start = x as i32;
                }
                num = num * 10 + pos.to_digit(10).unwrap()
            }
            if num_start != -1 && !pos.is_ascii_digit() {
                num_end = x as i32 - 1;
            }
            if num_start != -1 && x == x_size - 1 {
                num_end = x as i32;
            }

            if num_start != -1 && num_end != -1 {
                // println!("{num}, {num_start}:{num_end}");
                for xs in num_start..num_end + 1 {
                    numbers[y][xs as usize] = num as i32;
                }

                num = 0;
                num_start = -1;
                num_end = -1;
            }
        }
    }

    let mut sum = 0;
    let mut sum_gear_ratios = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, pos) in line.chars().enumerate() {
            if !pos.is_ascii_digit() && pos != '.' {
                let mut part_nums: Vec<i32> = Vec::new();
                for ys in y - 1..y + 2 {
                    for xs in x - 1..x + 2 {
                        let part_num = numbers[ys][xs];
                        if part_num != 0 && !part_nums.contains(&part_num) {
                            part_nums.push(part_num);
                            sum += part_num;
                        }
                    }
                }
                if pos == '*' {
                    if part_nums.len() == 2 {
                        let gear_ratio: i32 = part_nums[0] * part_nums[1];
                        sum_gear_ratios += gear_ratio;
                    }
                }
            }
        }
    }

    println!("Sum of part ids: {sum}");
    println!("Sum of gear ratios: {sum_gear_ratios}");
}
