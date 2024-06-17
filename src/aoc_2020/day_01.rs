/// Day 1, Part 1 -- https://adventofcode.com/2020/day/1
///
/// You're on vacation! Well, not yet, but soon you will be.
/// To pay a deposit on the place you're staying in, you need
/// to earn 50 stars. To earn your first star, the Elves need
/// you to fix your expense report (the puzzle input).
///
/// Specifically, to start off you need to find the two entries
/// in the list which add to 2020 and return their product.
pub fn find_2_sum_product(measures_input: &Vec<&str>) -> u32 {
  let measures: Vec<u32> = measures_input
    .into_iter()
    .map(|line| line.parse::<u32>().unwrap())
    .collect();

  for i in 0..measures_input.len() {
    for j in i + 1..measures_input.len() {
      if measures[i] + measures[j] == 2020 {
        println!("Sum 2020 pair: {} & {}", measures[i], measures[j]);
        return measures[i] * measures[j];
      }
    }
  }

  panic!("No solution found!");
}

/// Day 1, Part 2
///
/// The elves are thankful for the help, but they realized
/// that they gave you the wrong material. You got one gold star
/// for solving that, but you can get a second gold star if you
/// find what they actually need:
///
/// Instead of finding 2 numbers that sum to 2020, they actually need
/// the product of the 3 numbers that sum to 2020 instead.
pub fn find_3_sum_product(measures_input: &Vec<&str>) -> u32 {
  let measures: Vec<u32> = measures_input
    .into_iter()
    .map(|line| line.parse::<u32>().unwrap())
    .collect();

  for i in 0..measures_input.len() {
    for j in i + 1..measures_input.len() {
      for k in j + 1..measures_input.len() {
        if measures[i] + measures[j] + measures[k] == 2020 && i != j {
          println!("Sum 2020 pair: {} & {}", measures[i], measures[j]);
          return measures[i] * measures[j] * measures[k];
        }
      }
    }
  }

  panic!("No solution found!");
}
