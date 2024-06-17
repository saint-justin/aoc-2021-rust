use itertools::Itertools;
use std::collections::HashSet;

type Vector2D = (isize, isize);
const NORTH: Vector2D = (-1, 0);
const SOUTH: Vector2D = (1, 0);
const EAST: Vector2D = (0, 1);
const WEST: Vector2D = (0, -1);

/// Day 21, Part 1 -- https://adventofcode.com/2023/day/21
///
/// The airship drops you back off at Desert Island. On the airship,
/// one of the elves who works with the gardener heard you're good at
/// solving problems so he asks for your help with a problem. He has
/// a garden of his own that is either garden plots (.), his starting
/// position (S), or rocks (#). He's trying to figure out how many
/// garden plots he can reach in exactly 64 steps given that each step
/// he has to move somewhere (N/S/E/W) but can't stay in place.
///
/// How many locations can he access if he can only step on Garden Plots
/// and given that he needs to walk exactly 64 steps?
pub fn find_accessible_gardening_plots(garden_str: &Vec<&str>) -> usize {
  let mut garden = garden_str
    .iter()
    .map(|row| row.chars().collect_vec())
    .collect_vec();

  let mut positions: HashSet<Vector2D> = HashSet::new();
  let start_pos = get_start_pos(&garden);
  garden[start_pos.0 as usize][start_pos.1 as usize] = '.';
  positions.insert(start_pos);

  for _ in 0..64 {
    positions = step(&positions, &garden);
  }

  return positions.len();
}

fn get_start_pos(garden: &Vec<Vec<char>>) -> Vector2D {
  for i in 0..garden.len() {
    for j in 0..garden[i].len() {
      if 'S' == garden[i][j] {
        return (i as isize, j as isize);
      }
    }
  }
  panic!("No start position found in map.");
}

fn step(positions: &HashSet<Vector2D>, garden: &Vec<Vec<char>>) -> HashSet<Vector2D> {
  let mut new_positions: HashSet<Vector2D> = HashSet::new();
  for pos in positions {
    vec![NORTH, SOUTH, EAST, WEST].iter().for_each(|dir| {
      let new_pos = v_add(*dir, *pos);
      if pos_valid(new_pos, garden) {
        new_positions.insert(new_pos);
      }
    })
  }
  new_positions
}

fn pos_valid(pos: Vector2D, garden: &Vec<Vec<char>>) -> bool {
  if pos.0 < 0 || pos.1 < 0 || pos.0 >= garden.len() as isize || pos.1 >= garden[0].len() as isize {
    return false;
  }

  garden[pos.0 as usize][pos.1 as usize] == '.'
}

fn v_add(v1: Vector2D, v2: Vector2D) -> Vector2D {
  return (v1.0 + v2.0, v1.1 + v2.1);
}
