/// Day 15, Part 1 -- https://adventofcode.com/2023/day/15
/// 
/// Your reflector dish is sending all the light to the target
/// position but it looks like the light isn't being focused 
/// correctly at the target facility. Upon entry, you find a 
/// reindeer wearing goggles and a hardhat looking VERY panicked
/// next to a button that says "push for help".
/// 
/// The reindeer hands you a manual that describes how to start
/// the focusing process called a Holiday ASCII String Helper 
/// algorithm, or simply a HASH. The hash is relatively simple.
/// Your input is a comma delineated string containing multi-
/// character input lines. To HASH them, calculate the value
/// of the ASCII character, multiply it by 17, and return 
/// the remainder of it divided by 256. 
/// 
/// What is the sum of HASHing your inputs?
pub fn sum_hash_results(input: &Vec<&str>) -> u32 {
  input[0]
      .split(",")
      .map(|seq| {
          seq.chars()
              .into_iter()
              .fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
      })
      .sum()
}