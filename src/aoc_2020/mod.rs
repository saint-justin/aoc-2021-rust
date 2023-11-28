mod day_01;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
    match day {
        1 => {
            let two_sum_product = day_01::find_2_sum_product(input);
            let three_sum_product = day_01::find_3_sum_product(input);
            println!("\nDay 1!");
            println!("  Expense report 2-sum product:    {:?}", two_sum_product);
            println!("  Expense report 3-sum product:    {:?}", three_sum_product);
            // println!("  Windowed measurement increases: {:?}", windowed_increases);
        }
        _ => println!("Code for day {:?} undefined", day),
    }
}