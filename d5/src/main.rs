use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    file: PathBuf,
}

struct SeedMapping {
    source_start: i64,
    destination_start: i64,
    range: i64,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;
    let input = std::fs::read_to_string(&file).unwrap();

    let mut min_soil_id = i64::MAX;

    let mut seed_to_soil_map: Vec<SeedMapping> = Vec::new();

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let plant_data: Vec<i64> = first_line
        .replace("seeds: ", "")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut plant_ids: Vec<i64> = Vec::new();

    for (i, group) in plant_data.windows(2).enumerate() {
        if i % 2 == 0 {
            for id in group[0]..group[0] + group[1] {
                plant_ids.push(id);
            }
        }
    }

    let line_count = lines.clone().count();
    let mut parse_active = false;
    for (nr, line) in lines.enumerate() {
        if (line == "" || nr == line_count - 1) && parse_active {
            parse_active = false;
            min_soil_id = i64::MAX;

            for i in 0..plant_ids.len() {
                let id = plant_ids[i];
                let mut soil_id = -1;
                for seed_to_soil in &seed_to_soil_map {
                    if seed_to_soil.source_start <= id
                        && id < seed_to_soil.source_start + seed_to_soil.range
                    {
                        let offset = seed_to_soil.destination_start - seed_to_soil.source_start;
                        soil_id = id + offset;
                    }
                }
                if soil_id == -1 {
                    soil_id = id;
                }
                if soil_id < min_soil_id {
                    min_soil_id = soil_id;
                }
                plant_ids[i] = soil_id;
            }

            seed_to_soil_map = Vec::new();
        }
        if parse_active {
            let line_parts = line.split(' ').collect::<Vec<&str>>();
            let [destination_start, source_start, range] =
                <[&str; 3]>::try_from(line_parts).ok().unwrap();
            seed_to_soil_map.push(SeedMapping {
                source_start: source_start.parse::<i64>().unwrap(),
                destination_start: destination_start.parse::<i64>().unwrap(),
                range: range.parse::<i64>().unwrap(),
            });
        }
        if line.contains("-to-") {
            println!("{line}");
            parse_active = true;
        }
    }

    println!("Nearest location id: {min_soil_id}");
}
