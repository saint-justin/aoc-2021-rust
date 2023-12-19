use std::collections::HashSet;

use itertools::Itertools;

type Vector2D = (isize, isize);
const NORTH: Vector2D = (-1, 0);
const SOUTH: Vector2D = (1, 0);
const EAST: Vector2D = (0, 1);
const WEST: Vector2D = (0, -1);
fn v_add(v1: Vector2D, v2: Vector2D) -> Vector2D {
    (v1.0 + v2.0, v1.1 + v2.1)
}

/// Day 18, Part 1 -- https://adventofcode.com/2023/day/18
///
/// The factory needs a large supply of lava! The elves have written a
/// proposal for cutting a large pit to store the lava in (your input)
/// but they're not sure if it's big enough.
///
/// U 5 (#70c710)
/// L 8 (#0dc571)
/// D 5 (#5713f0)
/// R 3 (#d2c081)
/// U 1 (#70c710)
/// L 4 (#0dc571)
/// D 1 (#5713f0)
/// L 3 (#70c710)
///
/// The input is a set of trenches given as a direction and length pair
/// followed by an unimportant color for each given trench edge. Ignore it.
/// U 4 means up (north) 4 units then the next direction (L 8) means go left
/// 8 units from the previous position, and so on. The result of these
/// directions will be the pattern below (. is untouched, # is dug out):
///
/// Perimiter         Excavated
/// ............      ............
/// ..########..      ..########..
/// ..#......#..  ->  ..########..
/// ..#......#..  ->  ..########..
/// ..#.####.#..  ->  ..########..
/// ..###..###..      ..###..###..
/// ............      ............
///
/// After cutting the perimeter, the elves will also be removing all
/// the dirt inside the perimeter, resulting in a map like the one to the
/// right.
///
/// Each space the tractor drives over will empty out the space its on
/// before moving to the next space. If each space emptied is equal to
/// 1m^2 of space for lava storage, how many m^2 of space are emptied
/// according to the final input?
pub fn calculate_lava_volume(dig_plan: &Vec<&str>) -> usize {
    let mut edge_set: HashSet<Vector2D> = HashSet::new();
    let mut current_pos: Vector2D = (0, 0);
    edge_set.insert(current_pos);

    for instruction in dig_plan {
        let cleaned_instructions = instruction
            .chars()
            .filter(|ch| ch != &'(' && ch != &')')
            .join("");

        let (dir, distance, color) = cleaned_instructions
            .split(" ")
            .filter(|s| s != &"")
            .collect_tuple()
            .unwrap();

        let current_dir = match dir {
            "U" => NORTH,
            "D" => SOUTH,
            "L" => WEST,
            "R" => EAST,
            _ => panic!("Invalid direction argument passed: {}", dir),
        };

        for _ in 0..distance.parse::<usize>().unwrap() {
            current_pos = (current_pos.0 + current_dir.0, current_pos.1 + current_dir.1);
            edge_set.insert(current_pos);
        }
    }

    let mut to_explore: Vec<Vector2D> = vec![(1, 1)];
    let mut explored: Vec<Vector2D> = Vec::from_iter(edge_set.iter().map(|v| *v));

    while to_explore.len() > 0 {
        let current = to_explore.pop().unwrap();

        vec![
            v_add(current, NORTH),
            v_add(current, SOUTH),
            v_add(current, EAST),
            v_add(current, WEST),
        ]
        .iter()
        .for_each(|pos| {
            if !to_explore.contains(pos) && !explored.contains(pos) {
                to_explore.push(*pos);
                if explored.len() % 100 == 0 {
                    println!("Explored: {}", explored.len())
                }
            }
        });

        explored.push(current);
    }

    explored.len()
}
