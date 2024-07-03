mod day_01;

pub fn run_day_number(day: u32, input: &Vec<&str>) {
  match day {
    1 => {
      let fuel_sum = day_01::find_fuel_requirements(input);
      let recursive_fuel_sum = day_01::find_fuel_requirements_rec(input);
      println!("\nDay 01:");
      println!("  Fuel Requirement Sum:    {:?}", fuel_sum);
      println!("  Fuel Requirement Sum:    {:?}", recursive_fuel_sum);
    }

    _ => println!("Code for day {:?} undefined", day),
  }
}
