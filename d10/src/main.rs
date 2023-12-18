use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pos2 {
    x: i32,
    y: i32,
}

impl Pos2 {
    fn offset(&self, dx: i32, dy: i32) -> Pos2 {
        return Pos2 {
            x: self.x + dx,
            y: self.y + dy,
        };
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PipePointer {
    last: Pos2,
    pos: Pos2,
}

fn next_pos(p: Pos2, last: Pos2, pipe: &str) -> Pos2 {
    let (p1, p2) = match pipe {
        "-" => (p.offset(-1, 0), p.offset(1, 0)),
        "|" => (p.offset(0, -1), p.offset(0, 1)),

        "L" => (p.offset(0, -1), p.offset(1, 0)),
        "J" => (p.offset(0, -1), p.offset(-1, 0)),
        "7" => (p.offset(0, 1), p.offset(-1, 0)),
        "F" => (p.offset(0, 1), p.offset(1, 0)),

        "." => panic!(),
        "S" => panic!(),
        _ => panic!(),
    };

    if p1 == last {
        return p2;
    } else if p2 == last {
        return p1;
    } else {
        panic!();
    }
}

fn move_alone_pipe(pipe_pointer: PipePointer, grid: &Vec<Vec<String>>) -> PipePointer {
    let (pos, last) = (&pipe_pointer.pos, &pipe_pointer.last);
    let pipe = &grid[pos.y as usize][pos.x as usize];
    let next = next_pos(pos.clone(), last.clone(), pipe.as_ref());
    println!("{pipe} n: {:?}, p: {:?} l: {:?}", next, pos, last,);
    return PipePointer {
        pos: next,
        last: pos.clone(),
    };
}

fn get_grid(input: String) -> Vec<Vec<String>> {
    return input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
}

fn get_start_pipes(grid: &Vec<Vec<String>>) -> (PipePointer, PipePointer) {
    for (y, row) in grid.into_iter().enumerate() {
        for (x, pos) in row.into_iter().enumerate() {
            if pos == "S" {
                let (x, y) = (x as i32, y as i32);
                return (
                    PipePointer {
                        pos: Pos2 { x: x + 1, y },
                        last: Pos2 { x: x, y: y },
                    },
                    PipePointer {
                        pos: Pos2 { x: x, y: y + 1 },
                        last: Pos2 { x: x, y: y },
                    },
                );
            }
        }
    }
    panic!();
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let mut steps = 1;

    let grid: Vec<Vec<String>> = get_grid(input);
    let (mut pipe_1, mut pipe_2) = get_start_pipes(&grid);

    loop {
        pipe_1 = move_alone_pipe(pipe_1, &grid);
        pipe_2 = move_alone_pipe(pipe_2, &grid);

        steps += 1;

        if pipe_1.pos == pipe_2.pos {
            break;
        }
    }

    println!("Steps: {steps}");
}
