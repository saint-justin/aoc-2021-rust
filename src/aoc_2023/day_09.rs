use itertools::Itertools;

/// Day 9, Part 1 -- https://adventofcode.com/2023/day/9
///
/// You get to an oasis in the desert and find a downed hangglider.
/// It's really hot here so you're pretty sure that you can use the
/// hangglider to catch the next big draft up to your next destination,
/// but the heat pattern's are difficult to calculate in your head.
/// Instead, you've used your Oasis And Sand Instability Sensor (OASIS)
/// to take some readings. If you can extrapolate what the next
/// value in every reading is, you think you'll be able to find when
/// the next updraft is going to be.
///
/// What's the sum of the extrapolated next values?
pub fn extrapolate_pattern_sum(patterns: &Vec<&str>) -> i32 {
  let mut extrapolated_pattern_sum = 0;
  for pattern in patterns {
    let src_pattern = pattern
      .split(" ")
      .map(|s| s.parse::<i32>().unwrap())
      .collect_vec();

    let mut extrapolated_pattern = extrapolate_diff(&src_pattern);
    let mut patterns = vec![extrapolated_pattern.clone()];
    while !extrapolated_pattern.iter().all(|n| n == &0) {
      extrapolated_pattern = extrapolate_diff(&extrapolated_pattern);
      patterns.push(extrapolated_pattern.clone());
    }

    let pattern_addend: i32 = patterns.iter().map(|arr| arr.last().unwrap()).sum();
    extrapolated_pattern_sum += src_pattern.last().unwrap() + pattern_addend
  }

  return extrapolated_pattern_sum;
}

/// Day 9, Part 2
///
/// In retrospect, for safety's sake you should probably get more data.
/// Instead of waiting around and cooking under the hot sun, maybe you
/// should just extrapolate what the previous values would've been.
///
/// What's the sum of the previous extrapolated values?
pub fn extrapolate_pattern_sum_backward(patterns: &Vec<&str>) -> i32 {
  let mut extrapolated_pattern_sum: i32 = 0;
  for pattern in patterns {
    let src_pattern = pattern
      .split(" ")
      .map(|s| s.parse::<i32>().unwrap())
      .collect_vec();

    let mut extrapolated_pattern = extrapolate_diff(&src_pattern);
    let mut patterns = vec![extrapolated_pattern.clone()];
    while !extrapolated_pattern.iter().all(|n| n == &0) {
      extrapolated_pattern = extrapolate_diff(&extrapolated_pattern);
      patterns.push(extrapolated_pattern.clone());
    }

    let mut leading_values: Vec<i32> = vec![0];
    for row in (0..patterns.len()).rev() {
      let minus = patterns[row][0] - leading_values.last().unwrap();
      leading_values.push(minus);
    }

    extrapolated_pattern_sum += src_pattern.first().unwrap() - leading_values.last().unwrap();
  }

  return extrapolated_pattern_sum;
}

fn extrapolate_diff(src: &Vec<i32>) -> Vec<i32> {
  let mut diff: Vec<i32> = Vec::new();
  for i in 0..src.len() - 1 {
    diff.push(src[i + 1] - src[i]);
  }
  return diff;
}
