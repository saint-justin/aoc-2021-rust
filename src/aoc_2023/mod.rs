mod day_01;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
    match day {
        1 => {
            let calibration_sum = day_01::sum_calibration_values(input);
            let calibration_text_sum = day_01::sum_calibration_with_numbertext(input);
            println!("\nDay 1:");
            println!("  Calibration value sum:    {:?}", calibration_sum);
            println!("  Calibration text sum:     {:?}", calibration_text_sum);
        }
        _ => println!("Code for day {:?} undefined", day),
    }
}
