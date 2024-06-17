use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

/// Day 8, Part 1 -- https://adventofcode.com/2023/day/8
///
/// You're still riding your camel when a sandstorm approaches. Your
/// elf guide disappears which isn't super surprising given that she
/// just warned you about desert ghosts in the previous puzzle. Hoping
/// to quickly escape the network of "nodes" in the desert before the
/// sandstorm is upon you, you pull out a set of "maps" from a pouch
/// on your camel that define the desert's layout. You assume that
/// "AAA" should be where you are right now (your starting position)
/// and that "ZZZ" should be where you end up (your escape).
///
/// LLR
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
///
/// Your maps are given as a series of node inputs in the style formatted
/// above where "AAA" would be the start of a node and "BBB" and "CCC"
/// are the connected nodes to the left and right respectively. You also
/// receive a list of directions you're supposed to turn ("LLR") which
/// repeat once you've exhausted them. Starting at AAA, follow the
/// directions, repeating your L/R directions as needed, until you land
/// on position ZZZ. How many steps are required to reach ZZZ?
pub fn find_steps_to_zzz(directions: &Vec<&str>) -> usize {
  let lr_directions = directions[0].split("").filter(|s| s != &"").collect_vec();
  let map = parse_map(directions);

  let mut steps = 0;
  let mut current = "AAA";
  while current != "ZZZ" {
    let next_directions = map.get(current).unwrap();
    match lr_directions[steps % lr_directions.len()] {
      "L" => current = &next_directions.0,
      "R" => current = &next_directions.1,
      _ => panic!(
        "Invalid step passed: '{}'",
        lr_directions[steps % lr_directions.len()]
      ),
    }
    steps += 1;
  }

  return steps;
}

/// Day 8, Part 2
///
/// Uh oh, the sandstorm is closing distance and you aren't moving
/// anywhere. Maybe instead of being maps for humans, these are actually
/// maps for ghosts? Let's try it.
///
/// The way ghosts navigate, there are actually multiple entrances and
/// exits from the desert. Any location ending with an "A" is an entry
/// and any location ending with a "Z" can be considered an exit. As
/// a result of the way ghosts work, you need to simultaneosly nagivate
/// every single entry node until ALL of the positions you're tracking
/// are exit nodes. How many steps does it take until all nodes you're
/// on are exit nodes?
pub fn ghost_traverse_to_exit_steps(directions: &Vec<&str>) -> usize {
  let lr_directions = directions[0].split("").filter(|s| s != &"").collect_vec();
  let map = parse_map(directions);
  let mut positions = map
    .keys()
    .filter(|s| s.chars().last().unwrap() == 'A')
    .collect_vec();

  let mut steps = 0;

  let out = positions
    .iter()
    .map(|pos| {
      let mut all_positions: Vec<String> = Vec::new();
      let mut current = pos.clone();

      let mut step = 0;
      while !all_positions.contains(&format!(
        "{}_{}",
        current,
        lr_directions[step % lr_directions.len()]
      )) {
        // cache that it happened
        all_positions.push(format!(
          "{}_{}",
          current,
          lr_directions[step % lr_directions.len()]
        ));

        // step
        let next_directions = map.get(current).unwrap();
        match lr_directions[steps % lr_directions.len()] {
          "L" => current = &next_directions.0,
          "R" => current = &next_directions.1,
          _ => panic!(
            "Invalid step passed: '{}'",
            lr_directions[steps % lr_directions.len()]
          ),
        }
        step += 1;
      }

      println!("Found repeat for start {} at step {}", pos, step);
      return all_positions;
    })
    .collect_vec();

  for v in out {
    println!("{:?}", v);
  }

  return 0;
}

// Helpers
fn parse_map(directions: &Vec<&str>) -> HashMap<String, (String, String)> {
  let re = Regex::new(r"([A-Z]{3})").unwrap();
  let mut map = HashMap::new();
  for i in 2..directions.len() {
    let matches = re
      .find_iter(directions[i])
      .map(|m| m.as_str().to_owned())
      .collect_vec();
    map.insert(matches[0].clone(), (matches[1].clone(), matches[2].clone()));
  }
  return map;
}
