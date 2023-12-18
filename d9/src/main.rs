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

    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        // println!("{line}");
        let nums: Vec<i32> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        let mut diffs_container: Vec<Vec<i32>> = Vec::new();

        diffs_container.push(nums);

        let mut j = 0;
        loop {
            let mut diffs: Vec<i32> = Vec::new();

            let mut all_zero = true;
            for i in 0..diffs_container[j].len() - 1 {
                let diff = diffs_container[j][i + 1] - diffs_container[j][i];
                // print!("{diff}");
                if diff != 0 {
                    all_zero = false;
                }
                diffs.push(diff);
            }
            j += 1;
            diffs_container.push(diffs);
            // println!("");
            if all_zero {
                break;
            }
        }

        let len = diffs_container.len();
        diffs_container[len - 1].push(0);
        for i in 0..len - 1 {
            let last_diffs = &diffs_container[len - i - 2];
            let diffs = &diffs_container[len - i - 1];
            let diff = last_diffs.last().unwrap() + diffs.last().unwrap();
            // println!("{diff}");
            diffs_container[len - i - 2].push(diff);
        }

        let next_value = diffs_container[0].last().unwrap();
        println!("{next_value}");
        sum += next_value;
    }

    println!("Sum of next values: {sum}")
}

// y = x * c
// c = y / x

// c1   1^1 1^2 1^3   y1
// c2 * 2^2 2^2 2^3 = y2
// c3   3^3 3^2 3^3   y3
