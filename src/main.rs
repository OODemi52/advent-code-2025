use std::env;

mod day01;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }

    let day = &arguments[1];

    match day.as_str() {
        "1" => day01::solution::part1(),
        _ => eprintln!("Invalid day passed."),
    }
}
