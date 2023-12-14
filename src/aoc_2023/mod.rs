mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

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
            // TODO: Day 3, Part 2
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
            // TODO: Day 5, Part 2
        }

        6 => {
            let record_breaking_product = day_06::find_multisolution_product(input);
            let large_input_solutions = day_06::find_solution_large_input(input);
            println!("\nDay 6:");
            println!("  Record product:        {}", record_breaking_product);
            println!("  Record merged product: {}", large_input_solutions);
        }

        7 => {
            let total_winnings = day_07::find_camel_poker_winnings(input);
            let wild_winnings = day_07::hand_winnings_with_jokers(input);
            println!("\nDay 7:");
            println!("  Camel poker winnings (str):  {}", total_winnings);
            println!("  Camel poker winnings (wild): {}", wild_winnings);
        }

        8 => {
            let steps_to_zzz = day_08::find_steps_to_zzz(input);
            let ghost_steps_to_zzz = day_08::ghost_traverse_to_exit_steps(input);
            println!("\nDay 8:");
            println!("  Steps to ZZZ: {}", steps_to_zzz);
            println!("  Ghost steps to ZZZ: {}", ghost_steps_to_zzz);
        }

        9 => {
            let extrapolated_sum = day_09::extrapolate_pattern_sum(input);
            let extrapolated_rev_sum = day_09::extrapolate_pattern_sum_backward(input);
            println!("\nDay 9:");
            println!("  Extrapolated pattern sum: {}", extrapolated_sum);
            println!("  Extrapolated pattern sum: {}", extrapolated_rev_sum);
        }

        10 => {
            let furthest_section = day_10::find_furthest_loop_section(input);
            println!("\nDay 10:");
            println!("  Furthest section from loop steps: {}", furthest_section);
            // TODO:  Day 10, Part 2
        }

        11 => {
            let distance_sum = day_11::find_distance_sum(input);
            let scaled_distance_sum = day_11::find_scaled_distance_sum(input);
            println!("\nDay 11:");
            println!("  Sum of gap 02 distances: {}", distance_sum);
            println!("  Sum of gap 1m distances: {}", scaled_distance_sum);
        }

        12 => {
            let arrangement_sum = day_12::find_arrangement_sum(input);
            println!("\nDay 12:");
            println!("  Total possible arrangements: {}", arrangement_sum);
        }

        13 =>  {
            // let reflection_summary = day_13::find_reflection_summary(input);
            let smudged_summary = day_13::find_smudged_reflection_summary(input);
            println!("\nDay 13:");
            // println!("  Reflection summary: {}", reflection_summary);
            println!("  Smudged summary:    {}", smudged_summary);
        }

        _ => println!("Code for day {:?} undefined", day),
    }
}
