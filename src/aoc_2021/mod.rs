mod day_01;
mod day_02;
mod day_03;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
    match day {
        1 => {
            let increases = day_01::count_measurement_increases(&input);
            let windowed_increases = day_01::count_windowed_measurement_increases(&input);
            println!("\nDay 1:");
            println!("  Total measurement increases:    {:?}", increases);
            println!("  Windowed measurement increases: {:?}", windowed_increases);
        }
        2 => {
            let pos_product = day_02::find_positional_product(&input);
            let aimed_product = day_02::find_aimed_product(&input);
            println!("\nDay 2:");
            println!("  Positional product: {:?}", pos_product);
            println!("  Aimed product:      {:?}", aimed_product);
        }
        3 => {
            let power_consumption = day_03::get_submarine_power_consumption(&input);
            println!("\nDay 3:");
            println!("  Power consumption: {:?}", power_consumption);
        }
        _ => println!("Code for day {:?} undefined", day),
    }
}
