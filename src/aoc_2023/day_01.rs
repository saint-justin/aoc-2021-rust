use regex::Regex;
use std::ops::Deref;

/// Day 1, Part 1 -- https://adventofcode.com/2023/day/1
///
/// You're about to be fired out a trebuchet on your way to your newest
/// location when you realize that your team mishandled their calibration
/// data! Quick, find the first and last numbers in each string
/// and return their sum (a checksum) to make sure that their data
/// is still valid and that you aren't going to be thrown into a volcano!
pub fn sum_calibration_values(calibration_values: &Vec<&str>) -> u32 {
  let regex = Regex::new(r"[a-zA-Z]").unwrap();
  return calibration_values
    .iter()
    .map(|calibration_str| {
      let binding = regex.replace_all(calibration_str, "");
      let replaced = binding.deref();
      let ch_first = replaced.chars().nth(0).unwrap();
      let ch_last = replaced.chars().nth(replaced.len() - 1).unwrap();
      let n = format!("{}{}", ch_first, ch_last).parse::<u32>().unwrap();
      return n;
    })
    .sum();
}

/// Day 1, Part 2
///
/// Oh boy, just before you were going to finish validating your
/// checksum, you realized that some of the numbers are written
/// out as plaintext (e.g "one1one" should be counted as "111").
///
/// Given this new requirement, what's the new checksum?
pub fn sum_calibration_with_numbertext(calibration_values: &Vec<&str>) -> u32 {
  let regex = Regex::new(r"[a-zA-Z]").unwrap();
  return calibration_values
    .iter()
    .map(|calibration_str| {
      let nums_replaced_str = replace_ntext_with_nstr(calibration_str);
      let binding = regex.replace_all(&nums_replaced_str, "");
      let replaced = binding.deref();
      let ch_first = replaced.chars().nth(0).unwrap();
      let ch_last = replaced.chars().nth(replaced.len() - 1).unwrap();
      return format!("{}{}", ch_first, ch_last).parse::<u32>().unwrap();
    })
    .sum();
}

fn replace_ntext_with_nstr(calibration_string: &str) -> String {
  let nums_as_str: Vec<(&str, &str)> = vec![
    ("nineight", "98"),
    ("eighthree", "83"),
    ("eightwo", "82"),
    ("oneight", "18"),
    ("sevenine", "79"),
    ("twone", "21"), // these "special" replacements need to happen first
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
  ];

  let mut updated_str: String = calibration_string.to_owned();
  for (target, char) in nums_as_str {
    match updated_str.find(target) {
      Some(_) => updated_str = updated_str.replace(target, char),
      None => continue,
    }
  }

  return updated_str;
}
