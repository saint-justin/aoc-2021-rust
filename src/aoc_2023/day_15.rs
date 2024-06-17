use itertools::Itertools;
use regex::Regex;

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
  input[0].split(",").map(|seq| hash(seq)).sum()
}

/// Day 15, Part 2
///
/// The HASH algorithm set up up, but now you need to start the
/// HASH Manual Arrangement Process or HASHMAP. There are 256 boxes
/// labeled from 0 -> 255. The boxes are all arranged in a straight
/// line starting from where the light comes in from the reflectors
/// into the first box, then the second, and so on. Boxes are empty
/// until otherwise filled.
///
/// In each box is a lens slot that can keep one lens correctly
/// positioned inside the box. There's also a box of lenses labeled
/// from 1 to 9 along the side of the room. Each of the inputs in the
/// comma-delineated input are 2 or 3 parts.
///
/// rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
///
/// The three parts are the label (e.g. "rn"), the operation character
/// (e.g "-" or "="), and an optional focal length (1-9).
///
/// Running the HASH algorithm on the label tells you what box to
/// go to. Then, apply the operation denoted by the if the operation
/// character in the string. If the OpChar is a dash, go the the box
/// indicated by the first part and remove any lens with a label
/// the label in the input. In "cm-", you would check if any of the
/// lenses inside the box are labelled "cm" and remove any that exist
/// then slide all labels in a position below it up one to close the gap.
///
/// If the OpChar is an equals sign ("="), it will be followed by a 3rd
/// input that's the focal length. In "rn=1", the focal length is 1.
/// Take a lens matching the focal length and insert it into the box
/// matching the hash of the label. If there's already a lens with
/// that label in the box, replace it with the new one. If not, add it
/// behind all other lenses in the box.
///
/// Confirm the arrangement of all lenses by calculating sum of the
/// focusing power of each label. Focusing power is the result of
/// 1 * box_number * lens_slot for every lens with a given label.
/// For example, in the input above, the focusing power for labels
/// rn and cm would be
///
/// rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
/// cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
///
/// What's the focusing power of the entire lens array?
pub fn find_focusing_power(input: &Vec<&str>) -> u32 {
  let re_opchar = Regex::new(r"([-=]{1})").unwrap();
  let mut lens_boxes: Vec<OrderedMap<String, u32>> = vec![OrderedMap::new(); 256];

  for step in input[0].split(",") {
    let opchar = re_opchar.find(step).unwrap().as_str();
    let parts = step.split(opchar).filter(|s| s != &"").collect_vec();
    let hash = hash(parts[0]) as usize;
    match opchar {
      "=" => lens_boxes[hash].insert(parts[0].to_owned(), parts[1].parse::<u32>().unwrap()),
      "-" => lens_boxes[hash].remove(parts[0].to_owned()),
      _ => panic!("Invalid operation character!"),
    };
  }

  let mut focusing_power = 0;
  for i in 0..lens_boxes.len() {
    for j in 0..lens_boxes[i].pairs.len() {
      focusing_power += (i + 1) as u32 * (j + 1) as u32 * lens_boxes[i].pairs[j].1;
    }
  }

  return focusing_power;
}

#[derive(Debug, Clone)]
struct OrderedMap<T: std::cmp::PartialEq + Clone, U: std::cmp::PartialEq + Clone> {
  pairs: Vec<(T, U)>,
}

impl<T: std::cmp::PartialEq + Clone, U: std::cmp::PartialEq + Clone> OrderedMap<T, U> {
  pub fn new() -> OrderedMap<T, U> {
    OrderedMap { pairs: Vec::new() }
  }

  pub fn keys(&self) -> Vec<T> {
    self.pairs.iter().map(|p| p.0.clone()).collect_vec()
  }

  pub fn insert(&mut self, key: T, value: U) {
    if self.keys().contains(&key) {
      let i = self.key_index(key);
      self.pairs[i].1 = value;
    } else {
      self.pairs.push((key, value));
    }
  }

  pub fn remove(&mut self, key: T) {
    if self.keys().contains(&key) {
      let removed_index = self.key_index(key);
      self.pairs.remove(removed_index);
    }
  }

  fn key_index(&mut self, key: T) -> usize {
    self
      .pairs
      .iter()
      .enumerate()
      .filter(|(_, (k, _))| k == &key)
      .collect_vec()[0]
      .0
  }
}

// HASH the value of a given string
fn hash(s: &str) -> u32 {
  s.chars()
    .into_iter()
    .fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
}
