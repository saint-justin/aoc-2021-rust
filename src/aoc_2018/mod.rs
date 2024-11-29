mod day_01;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
  match day {
    1 => {
      let frequency = day_01::find_frequency(input);
      println!("\nDay 01:");
      println!("  Result Frequency:    {:?}", frequency);
    }

    _ => println!("Code for day {:?} undefined", day),
  }
}