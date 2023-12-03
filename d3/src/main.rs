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

    let mut parts: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let mut parts_row: Vec<bool> = Vec::new();
        for pos in line.chars() {
            let is_part = !pos.is_ascii_digit() && pos != '.';
            parts_row.push(is_part);
        }
        parts.push(parts_row);
    }

    let y_size = input.lines().collect::<Vec<&str>>().len();
    for (y, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut num_start: i32 = -1;
        let mut num_end: i32 = -1;
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

                let mut is_part = false;
                let y = y as i32;
                for ys in y - 1..y + 2 {
                    for xs in num_start - 1..num_end + 2 {
                        // println!("{is_inside_grid} x:{xs} y:{ys}");
                        if xs >= 0 && xs < x_size as i32 && ys >= 0 && ys < y_size as i32 {
                            if parts[ys as usize][xs as usize] {
                                is_part = true;
                            }
                        }
                    }
                }
                if is_part {
                    // println!("Part:{num}");
                    sum += num;
                }

                num = 0;
                num_start = -1;
                num_end = -1;
            }
        }
    }

    println!("Sum of part ids: {sum}"); // 519444
}
