mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
    match day {
        1 => {
            let calibration_sum = day_01::sum_calibration_values(input);
            let calibration_text_sum = day_01::sum_calibration_with_numbertext(input);
            println!("\nDay 1:");
            println!("  Calibration value sum:    {:?}", calibration_sum);
            println!("  Calibration text sum:     {:?}", calibration_text_sum);
        }

        2 => {
            let valid_id_sum = day_02::possible_game_id_sum(input);
            let power_sum = day_02::find_power_sum(input);
            println!("\nDay 2:");
            println!("  Valid ID sum:  {:?}", valid_id_sum);
            println!("  Power sum:     {:?}", power_sum);
        }

        3 => {
            let valid_parts_sum = day_03::valid_parts_sum(input);
            println!("\nDay 3:");
            println!("  Valid parts sum: {:?}", valid_parts_sum);
        }

        4 => {
            let scratcher_points = day_04::calculate_scratcher_points(input);
            let scratcher_cards = day_04::sum_total_scratchers(input);
            println!("\nDay 4:");
            println!("  Scratcher points: {}", scratcher_points);
            println!("  Scratcher cards:  {}", scratcher_cards);
        }

        5 => {
            let lowest_seed_local = day_05::find_lowest_initial_seed_location(input);
            println!("\nDay 5:");
            println!("  Lowest initial seed location: {}", lowest_seed_local);
        }

        6 => {
            let record_breaking_product = day_06::find_multisolution_product(input);
            let large_input_solutions = day_06::find_large_input_solutions(input);
            println!("\nDay 6:");
            println!(
                "  Lowest record-breaking product: {}",
                record_breaking_product
            );
            println!(
                "  Lowest record-breaking product: {}",
                large_input_solutions
            );
        }

        _ => println!("Code for day {:?} undefined", day),
    }
}
