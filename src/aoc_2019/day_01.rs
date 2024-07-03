/// Day 1, Part 1 -- https://adventofcode.com/2019/day/1
/// 
/// You're given a list of spaceship module masses and you need
/// to convert them into a quantity of fuel required to power
/// that ship. 
/// 
/// How much fuel do you need to for the mass of all the modules
/// on your spaceship?
pub fn find_fuel_requirements(modules: &Vec<&str>) -> i32 {
  modules.iter()
    .map(|mass| (mass.parse::<i32>().unwrap() / 3) - 2)
    .sum()
}

pub fn find_fuel_requirements_rec(modules: &Vec<&str>) -> i32 {
  modules.iter()
    .map(|mass| recursive_fuel_req_for_mass(mass.parse::<i32>().unwrap()))
    .sum()
}

fn recursive_fuel_req_for_mass(mass: i32) -> i32 {
  let remainder = (mass / 3) - 2;
  if remainder <= 2 {
    return std::cmp::max(remainder, 0)
  }
  return remainder + recursive_fuel_req_for_mass(remainder)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_condition_works() {
      let result = recursive_fuel_req_for_mass(14);
      assert_eq!(result, 2);
    }

    #[test]
    fn small_value_works() {
      let result = recursive_fuel_req_for_mass(1969);
      assert_eq!(result, 966);
    }

    #[test]
    fn big_value_works() {
      let result = recursive_fuel_req_for_mass(100756);
      assert_eq!(result, 50346);
    }
}