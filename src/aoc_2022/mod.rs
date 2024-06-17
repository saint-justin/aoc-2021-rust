mod day_01;
mod day_02;
mod day_03;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
  match day {
    1 => {
      let most_calories = day_01::find_elf_carrying_most_calories(input);
      let top_caloriie_sum = day_01::find_top_three_calorie_sum(input);
      println!("\nDay 1:");
      println!("  Max calorie elf:   {:?}", most_calories);
      println!("  Top 3 calorie sum: {:?}", top_caloriie_sum);
    }

    2 => {
      let total_rps_score = day_02::calculate_total_score(input);
      let adjusted_rps_score = day_02::calculate_total_score_adjusted(input);
      println!("\nDay 2:");
      println!("  Total RPS score:    {:?}", total_rps_score);
      println!("  Adjusted RPS score: {:?}", adjusted_rps_score);
    }

    3 => {
      let dup_priority_sum = day_03::find_priority_sum_of_dups(input);
      let badge_priority_sum = day_03::find_priority_sum_of_team_badges(input);
      println!("\nDay 3:");
      println!("  Duplicate priority sum:  {:?}", dup_priority_sum);
      println!("  Team badge priority sum: {:?}", badge_priority_sum);
    }

    // TODO: Day 4
    5 => {
      let rearr_msg = day_05::find_rearrangement_message(input);
      let multimove_msg = day_05::find_multimove_message(input);
      println!("\nDay 5:");
      println!("  Rearrangement message: {:?}", rearr_msg);
      println!("  Multi-move message:    {:?}", multimove_msg);
    }

    6 => {
      let chars_to_packet = day_06::characters_before_start_of_packet(input);
      let chars_to_packet_big = day_06::characters_before_start_of_packet_big(input);
      println!("\nDay 6:");
      println!("  Characters to packet:     {:?}", chars_to_packet);
      println!("  Characters to big packet: {:?}", chars_to_packet_big);
    }

    7 => {
      let find_dir_sum = day_07::find_sum_dirs_under_100000(input);
      println!("\nDay 7:");
      println!("  Directory Sum Under 10k: {:?}", find_dir_sum);
    }

    8 => {
      let visible_trees = day_08::get_visible_tree_count(input);
      let most_scenic_tree_score = day_08::get_visible_tree_count(input);
      println!("\nDay 8:");
      println!("  Total visible trees: {:?}", visible_trees);
      println!("  Most scenic tree sum: {:?}", most_scenic_tree_score);
    }

    _ => println!("Code for day {:?} undefined", day),
  }
}
