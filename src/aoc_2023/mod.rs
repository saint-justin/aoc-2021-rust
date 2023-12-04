mod day_01;
mod day_02;
mod day_03;

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
        _ => println!("Code for day {:?} undefined", day),
    }
}
