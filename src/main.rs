use std::{env, fs};

mod day_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    if args.len() > 2 {
        panic!("Invalid args found, day number must passed")
    }

    let day_number = args[1].parse::<u32>().unwrap();
    let day_path = format!("./src/inputs/day_{:02}.txt", day_number);
    println!("day_path: {:?}", day_path);

    let input_contents = fs::read_to_string(day_path)
      .expect("Should have been able to read the file at path {:?}");
    
    let puzzle_input: Vec<&str> = input_contents
        .split(['\n'])
        .map(|e| e.trim())
        .collect();
    
    match day_number {
        1 => {
            let increases = day_01::solutions::count_measurement_increases(&puzzle_input);
            let windowed_increases = day_01::solutions::count_windowed_measurement_increases(&puzzle_input);
            println!("\nDay 1:");
            println!("  Total measurement increases:          {:?}", increases);
            println!("  Total windowed measurement increases: {:?}", windowed_increases);
        },
        2 => println!("Running Day 2!"),
        _ => println!("Code for day {:?} undefined", day_number),
    }
}
