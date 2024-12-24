mod day_1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "day_11" => day_1::part1(args.get(2).unwrap()),
            "day_12" => day_1::part2(args.get(2).unwrap()),
            _ => {
                println!("Invalid argument {}", args[1]);
            }
        };
    };
}
