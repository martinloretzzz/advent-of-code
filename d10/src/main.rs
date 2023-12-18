use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn move_pos(p: Vec2, dx1: i32, dy1: i32, dx2: i32, dy2: i32) -> (Vec2, Vec2) {
    return (
        Vec2 {
            x: p.x + dx1,
            y: p.y + dy1,
        },
        Vec2 {
            x: p.x + dx2,
            y: p.y + dy2,
        },
    );
}

fn next_pos_options(p: Vec2, pipe: &str) -> (Vec2, Vec2) {
    return match pipe {
        "-" => move_pos(p, -1, 0, 1, 0),
        "|" => move_pos(p, 0, -1, 0, 1),

        "L" => move_pos(p, 0, -1, 1, 0),
        "J" => move_pos(p, 0, -1, -1, 0),
        "7" => move_pos(p, 0, 1, -1, 0),
        "F" => move_pos(p, 0, 1, 1, 0),

        "." => panic!(),
        "S" => panic!(),
        _ => panic!(),
    };
}

fn next_pos_from_options((p1, p2): (Vec2, Vec2), last: Vec2) -> Vec2 {
    if p1 == last {
        return p2;
    } else if p2 == last {
        return p1;
    } else {
        panic!();
    }
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let lines = input.lines();

    let mut grid: Vec<Vec<&str>> = Vec::new();

    let mut last_pos_1 = Vec2 { x: 0, y: 0 };
    let mut last_pos_2 = Vec2 { x: 0, y: 0 };
    let mut pos_1 = Vec2 { x: 0, y: 0 };
    let mut pos_2 = Vec2 { x: 0, y: 0 };

    let mut steps = 1;

    for (y, line) in lines.enumerate() {
        // println!("{line}");
        let row: Vec<&str> = line.split("").filter(|s| !s.is_empty()).collect();

        for (x, pos) in row.clone().into_iter().enumerate() {
            if pos == "S" {
                let x = x as i32;
                let y = y as i32;

                let s = Vec2 { x: x, y: y };
                last_pos_1 = s.clone();
                last_pos_2 = s.clone();

                pos_1 = Vec2 { x: x + 1, y };
                pos_2 = Vec2 { x: x, y: y + 1 };
            }
        }

        grid.push(row);
    }

    loop {
        let pipe_1 = grid[pos_1.y as usize][pos_1.x as usize];
        let next_1 =
            next_pos_from_options(next_pos_options(pos_1.clone(), pipe_1), last_pos_1.clone());
        println!(
            "1: {pipe_1} n: {:?}, p: {:?} l: {:?}",
            next_1, pos_1, last_pos_1,
        );
        last_pos_1 = pos_1.clone();
        pos_1 = next_1.clone();

        let pipe_2 = grid[pos_2.y as usize][pos_2.x as usize];
        let next_2 =
            next_pos_from_options(next_pos_options(pos_2.clone(), pipe_2), last_pos_2.clone());
        println!(
            "2: {pipe_2} n: {:?}, p: {:?} l: {:?}",
            next_2, pos_2, last_pos_2
        );
        last_pos_2 = pos_2.clone();
        pos_2 = next_2.clone();

        steps += 1;
        if pos_1 == pos_2 {
            break;
        }
    }

    println!("Steps: {steps}");
}
