fn main() {
    let times = [56, 97, 78, 75];
    let records = [546, 1927, 1131, 1139];
    let mut product = 1;
    for game_id in 0..times.len() {
        let time = times[game_id];
        let record = records[game_id];
        let mut combinations = 0;
        for i in 0..time {
            let distance = i * (time - i);
            if distance > record {
                combinations += 1;
            }
        }
        println!("{combinations}");
        product *= combinations;
    }
    println!("{product}");
}
