use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

fn is_part_number(item: &char) -> bool {
    return item.is_ascii_digit();
}

fn is_part(item: &char) -> bool {
    return !item.is_ascii_digit() && *item != '.';
}

fn is_gear(item: &char) -> bool {
    return *item == '*';
}

fn get_sourounding_part_numbers(number_grid: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<i32> {
    let mut part_numbers: Vec<i32> = Vec::new();
    for ys in y - 1..y + 2 {
        for xs in x - 1..x + 2 {
            let part_num = number_grid[ys][xs];
            if part_num != 0 && !part_numbers.contains(&part_num) {
                part_numbers.push(part_num);
            }
        }
    }
    return part_numbers;
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let size = input.lines().collect::<Vec<&str>>().len();
    let mut numbers: Vec<Vec<i32>> = vec![vec![0; size]; size];

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (y, grid_row) in grid.iter().enumerate() {
        let (mut num, mut num_start) = (0, 0);

        for (x, pos) in grid_row.iter().enumerate() {
            let x_size = grid_row.len();
            if is_part_number(&pos) {
                if num_start == -1 {
                    num_start = x as i32;
                }
                num = num * 10 + pos.to_digit(10).unwrap()
            }
            if num_start != -1 && (x == x_size - 1 || !is_part_number(&grid_row[x + 1])) {
                let num_end = x as i32;
                // println!("{num}, {num_start}:{num_end}");
                for xs in num_start..num_end + 1 {
                    numbers[y][xs as usize] = num as i32;
                }

                num = 0;
                num_start = -1;
            }
        }
    }

    let mut sum = 0;
    let mut sum_gear_ratios = 0;
    for (y, grid_row) in grid.iter().enumerate() {
        for (x, pos) in grid_row.iter().enumerate() {
            if is_part(&pos) {
                let part_numbers = get_sourounding_part_numbers(&numbers, x, y);
                if is_gear(&pos) {
                    if part_numbers.len() == 2 {
                        let gear_ratio = part_numbers[0] * part_numbers[1];
                        sum_gear_ratios += gear_ratio;
                    }
                }
                sum += part_numbers.iter().sum::<i32>();
            }
        }
    }

    println!("Sum of part ids: {sum}");
    println!("Sum of gear ratios: {sum_gear_ratios}");
}
