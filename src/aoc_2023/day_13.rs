use itertools::Itertools;

/// Day 13, Part 1 -- https://adventofcode.com/2023/day/13
///
/// You've made it to lava island, but it turns out it's REALLY
/// difficult to get everywhere because this place is completely
/// full of mirrors. Anywhere you look you see one gigantic
/// mirror or another which makes it really hard to tell whether
/// you're about to step onto a rock or into a laval stream.
///
/// To help solve this issue, you took a collection of patterns
/// of ash (.) and rock (#) that you see as you walk. By analyzing
/// the ash and rock, you can tell where mirrors are by finding
/// what part of the samples are reflected.
///
///   Ex_1:        Ex_2:
///  123456789
///  #.##..##.   1 #...##..# 1
///  ..#.##.#.   2 #....#..# 2
///  ##......#   3 ..##..### 3
///  ##......#   4 #####.##. 4
///  ..#.##.#.   5 #####.##. 5
///  ..##..##.   6 ..##..### 6
///  #.#.##.#.   7 #....#..# 7
///  123456789
///
/// For example, in Ex_1 above, the inflection point is between
/// characters 5 & 6 on the horizontal axis. In addition, in
/// Ex_2 you can find the inflection point between rows 4 & 5.
///
/// For each given pattern, if a reflection is over a vertical
/// axis, add the amount of rows to the left of it to the point
/// sum. For each horizontal reflection, add the amount of rows
/// above it multiplied by 100 to the point sum. What's the
/// point total of all patterns in the input?
pub fn find_reflection_summary(all_patterns: &Vec<&str>) -> u32 {
  let mut separated_maps: Vec<Vec<String>> = Vec::new();
  let mut temp_map: Vec<String> = Vec::new();
  for line in all_patterns {
    if line == &"" {
      separated_maps.push(temp_map.clone());
      temp_map = Vec::new();
    } else {
      temp_map.push(line.to_owned().to_owned())
    }
  }

  // Check each map and calculate its point value by its inflection point
  separated_maps
    .iter()
    .map(|pattern| find_inflection_point(pattern))
    .enumerate()
    .fold(0, |acc, (i, ip)| {
      println!("Pattern {}: {:?}", i, ip);
      acc + calculate_value_of_inflection_point(&ip)
    })
}

/// Day 13, Part 2
///
/// Ok so the plan was going great from part one right until you walked
/// DIRECTLY into another mirror. It turns out, each of the input
/// patterns have a single smudge meaning that one position that should
/// be a (.) is actually a (#) or vice versa. Given this new information,
/// locate and fix the smudge that causes a different reflection line
/// to be valid and use the fixed maps to re-calculate the point sum
/// of all pattern reflections.
///
/// What's the new point sum?
pub fn find_smudged_reflection_summary(all_patterns: &Vec<&str>) -> u32 {
  let mut separated_maps: Vec<Vec<String>> = Vec::new();
  let mut temp_map: Vec<String> = Vec::new();
  for line in all_patterns {
    if line == &"" {
      separated_maps.push(temp_map.clone());
      temp_map = Vec::new();
    } else {
      temp_map.push(line.to_owned().to_owned())
    }
  }

  let inflection_points = separated_maps
    .iter()
    .map(|pattern| find_and_replace_smudge(pattern))
    .map(|pattern: Vec<String>| find_inflection_point(&pattern))
    .collect_vec();

  println!("All inflection points found: {:?}", inflection_points);

  inflection_points
    .iter()
    .enumerate()
    .map(|(i, ip)| {
      println!("Pattern {}: {:?}", i, ip);
      let v = calculate_value_of_inflection_point(&ip);
      println!("IP Value: {}", v);
      v
    })
    .sum()
}

// vvv  Helper functions  vvv
fn find_and_replace_smudge(pattern: &Vec<String>) -> Vec<String> {
  let mismatch;
  let opt_mismatch = find_smudge(pattern);
  if opt_mismatch.is_some() {
    mismatch = opt_mismatch.unwrap();
  } else {
    let pattern_transpose = transpose_pattern(pattern);
    mismatch = find_smudge(&pattern_transpose).unwrap();
  }

  println!("Found a mismatch! {:?}", mismatch);
  let mut new_pattern = pattern.clone();
  new_pattern[mismatch.row].replace_range(
    mismatch.col..mismatch.col + 1,
    if pattern[mismatch.row].chars().nth(mismatch.col).unwrap() == '#' {
      "."
    } else {
      "#"
    },
  );

  println!("\nBase pattern:");
  for line in pattern {
    println!("{}", line);
  }

  println!("\nNew pattern:");
  for line in &new_pattern {
    println!("{}", line);
  }
  return new_pattern;
}

// Returns a list of all horizontal + vertical inflection points across a pattern
fn find_inflection_point(pattern: &Vec<String>) -> InflectionPoint {
  let mut inflection_points: Vec<InflectionPoint> = Vec::new();
  let pattern_transpose = transpose_pattern(pattern);

  get_inflection_positions(pattern)
    .iter()
    .for_each(|pos| inflection_points.push(InflectionPoint::new(ReflectionType::Horizontal, *pos)));

  get_inflection_positions(&pattern_transpose)
    .iter()
    .for_each(|pos| inflection_points.push(InflectionPoint::new(ReflectionType::Vertical, *pos)));

  println!("Inflection point found!");
  inflection_points[0]
}

// Returns any horizontal inflection point across a given pattern
fn get_inflection_positions(pattern: &Vec<String>) -> Vec<usize> {
  let mut inflection_points: Vec<usize> = Vec::new();
  for i in 1..pattern[0].len() {
    let max_width = pattern[0].len() / 2;
    if pattern.iter().all(|s| {
      let s1 = if i <= max_width { &s[0..i] } else { &s[i..] };
      let s2 = if i <= max_width {
        &s[i..i * 2]
      } else {
        &s[s.len() - (s1.len() * 2)..s.len() - s1.len()]
      };
      s1 == s2.chars().rev().collect::<String>()
    }) {
      inflection_points.push(i)
    }
  }
  return inflection_points;
}

// Returns any horizontal inflection point across a given pattern
fn find_smudge(pattern: &Vec<String>) -> Option<Mismatch> {
  for i in 1..pattern[0].len() {
    let mut mismatches: Vec<Mismatch> = Vec::new();
    let max_width = pattern[0].len() / 2;
    for s in pattern {
      let s1 = if i <= max_width { &s[0..i] } else { &s[i..] };
      let s2: &str = if i <= max_width {
        &s[i..i * 2]
      } else {
        &s[s.len() - (s1.len() * 2)..s.len() - s1.len()]
      };
      for j in 0..s1.len() {
        if s1.chars().nth(j) != s2.chars().rev().nth(j) {
          mismatches.push(Mismatch::new(i - 1, j))
        }
      }
    }

    // println!("  Mismatches found: {:?}", mismatches);
    if mismatches.len() == 1 {
      println!("Mismatch in reflection: {:?}", mismatches[0]);
      return Some(mismatches[0]);
    }
  }

  return None;
}

// Transposes a given 2D matrix (via StackOverflow):
// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust)
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!v.is_empty());
  let len = v[0].len();
  let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| n.next().unwrap())
        .collect::<Vec<T>>()
    })
    .collect()
}

fn transpose_pattern(pattern: &Vec<String>) -> Vec<String> {
  let pattern_2d = pattern
    .iter()
    .map(|s| s.chars().map(|ch| ch.to_string()).collect_vec())
    .collect_vec();

  transpose(pattern_2d)
    .iter()
    .map(|v| v.join(""))
    .collect_vec()
}

// Calculates the point value of a given inflection point
fn calculate_value_of_inflection_point(ip: &InflectionPoint) -> u32 {
  match ip.r_type {
    ReflectionType::Horizontal => ip.index as u32,
    ReflectionType::Vertical => ip.index as u32 * 100_u32,
  }
}

#[derive(Debug, Clone, Copy)]
enum ReflectionType {
  Horizontal,
  Vertical,
}

#[derive(Debug, Clone, Copy)]
struct InflectionPoint {
  r_type: ReflectionType,
  index: usize,
}

impl InflectionPoint {
  pub fn new(r_type: ReflectionType, index: usize) -> InflectionPoint {
    InflectionPoint { r_type, index }
  }
}

#[derive(Debug, Copy, Clone)]
struct Mismatch {
  row: usize,
  col: usize,
}

impl Mismatch {
  pub fn new(row: usize, col: usize) -> Mismatch {
    Mismatch { row, col }
  }
}
