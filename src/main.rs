use std::{env, fs};

mod aoc_2020;
mod aoc_2021;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    if args.len() > 3 {
        panic!("Invalid args found, day number must passed")
    }

    let year_number = args[1].parse::<u32>().unwrap();
    let day_number = args[2].parse::<u32>().unwrap();
    let input_path = format!("./src/inputs/{}/day_{:02}.txt", year_number, day_number);
    println!("input_path: {:?}", input_path);

    let input_contents = fs::read_to_string(input_path)
        .expect("Should have been able to read the file at path {:?}");

    let puzzle_input: Vec<&str> = input_contents
        .split(['\n'])
        .map(|e| e.trim())
        .collect();

    match year_number {
        2020 => aoc_2020::run_day_number(day_number, &puzzle_input),
        2021 => aoc_2021::run_day_number(day_number, &puzzle_input),
        _ => panic!("Invalid year number passed: [{:?}]", year_number)
    }
}
