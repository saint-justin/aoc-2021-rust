mod day_01;
mod day_02;
mod day_03;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
    match day {
        1 => {
            let two_sum_product = day_01::find_2_sum_product(input);
            let three_sum_product = day_01::find_3_sum_product(input);
            println!("\nDay 1!");
            println!("  Expense report 2-sum product:    {:?}", two_sum_product);
            println!("  Expense report 3-sum product:    {:?}", three_sum_product);
        }

        2 => {
            let valid_by_count = day_02::find_valid_passwords_by_count(input);
            let valid_by_position = day_02::find_valid_passwords_by_position(input);

            println!("\n Day 2!");
            println!("   Valid passwords by char count: {:?}", valid_by_count);
            println!("   Valid passwords by char pos:   {:?}", valid_by_position);
        }

        3 => {
            let straight_line_collisions = day_03::count_tree_collisions(input);
            let collision_product = day_03::count_tree_collision_product(input);
            println!("\n Day 3!");
            println!(
                "  Straight line colisions:     {:?}",
                straight_line_collisions
            );
            println!("  Multiple collisions product: {:?}", collision_product);
        }

        _ => println!("Code for day {:?} undefined", day),
    }
}
