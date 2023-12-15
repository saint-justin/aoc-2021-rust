use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);

/// Day 14, Part 1 -- https://adventofcode.com/2023/day/14
///
/// You made it up and realized that all those whacky mirrors are
/// actually individual pieces of a huge mirror array used to heat
/// up rocks to make lava! Upon closer inspection, you can adjust
/// all of the dishes to focus it four directions, N, S, E, W.
///
/// Pre-tilt         Post-tilt
/// O....#....  -->  OOOO.#.O..
/// O.OO#....#       OO..#....#
/// .....##...       OO..O##..O
/// OO.#O....O  -->  O..#.OO...
/// .O.....O#.       ........#.
/// O.#..O.#.#       ..#....#.#
/// ..O..#O..O  -->  ..O..#.O.O
///
/// Given a layout of rocks like the one shown above, by tilting
/// the mirror array in one direction, the rocks on top of it will
/// also move. All items on the map can be assumed to be a rock
/// that can move (O), a rock that can't move (#), or an empty
/// space (.). If you were to tilt the array north, all the rocks
/// that can move would shift north until they hit a rock that can't
/// move or until they hit the perimeter support beams. Those support
/// beans also look pretty flimsy, it would be best to make sure they
/// won't break.
///
/// Validate the total load on the north support beams by calculating
/// how many moving rocks would rest on it after tilting the array
/// north. The load of any one given rock is calculated by measuring
/// how many rows it is from the southern support beam including
/// its own row. What's the total load on the north support beam
/// after tilting the array to roll the loose rocks north?
pub fn calculate_north_load(initial_map: &Vec<&str>) -> u32 {
    let map_vec = initial_map
        .iter()
        .map(|row| {
            row.split("")
                .filter(|s| s != &"")
                .map(|s| s.to_owned())
                .collect_vec()
        })
        .collect_vec();

    return calculate_load(&shift_rocks(&map_vec, NORTH));
}

/// Day 14, Part 2 -- https://adventofcode.com/2023/day/14#part2
///
/// You succesfully deformed the reflector dish, but it didn't
/// focus the bean in the way you want. It turns out, you actually
/// should be tilting the dish according to a "spin cycle" in which
/// you tilt the platform four times (N, W, S, E). After each tilt,
/// all rocks move in the direction the platform tilt. This process
/// should work, if we just run it a bunch of times right?
///
/// What's the total load on the north support beams after running
/// the spin cycle 1,000,000,000 (1 billion) times?
pub fn calculate_north_load_after_1b_cycles(initial_map: &Vec<&str>) -> u32 {
    let map_vec = initial_map
        .iter()
        .map(|row| {
            row.split("")
                .filter(|s| s != &"")
                .map(|s| s.to_owned())
                .collect_vec()
        })
        .collect_vec();

    let spin_cycle_directions = vec![NORTH, WEST, SOUTH, EAST];
    let mut hashmap: HashMap<Vec<Vec<String>>, usize> = HashMap::new();
    let mut shifted_map = map_vec.clone();

    for i in 0..1_000_000_000 {
        for direction in &spin_cycle_directions {
            shifted_map = shift_rocks(&shifted_map, *direction)
        }

        if hashmap.contains_key(&shifted_map) {
            let initial_index: usize = *hashmap.get(&shifted_map).unwrap();
            let cycle_length = i - initial_index;
            let remaining = (1_000_000_000 - i) % cycle_length;
            let out_index = initial_index + ((i - initial_index + remaining) % cycle_length) - 1;
            shifted_map = hashmap
                .clone()
                .iter()
                .find(|(_k, v)| v == &&out_index)
                .unwrap()
                .0
                .to_owned();

            break;
        } else {
            hashmap.insert(shifted_map.clone(), i);
        }
    }

    return calculate_load(&shifted_map);
}

/* --- Helper functions --- */
fn shift_rocks(initial_map: &Vec<Vec<String>>, direction: (isize, isize)) -> Vec<Vec<String>> {
    let mut map: Vec<Vec<String>> = initial_map
        .clone()
        .iter()
        .map(|row| row.iter().map(|s| s.to_owned().to_owned()).collect_vec())
        .collect_vec();

    let mut movable_rocks: Vec<(isize, isize)> = Vec::new();
    for row in 0..initial_map.len() {
        for col in 0..initial_map[0].len() {
            if initial_map[row][col] == "O" {
                movable_rocks.push((row as isize, col as isize));
                map[row][col] = ".".to_owned();
            }
        }
    }

    movable_rocks.sort_by(|r1, r2| compare_rocks(r1, r2, direction));
    for rock in movable_rocks {
        let new_pos = shift_rock(rock, &map, direction);
        map[new_pos.0 as usize][new_pos.1 as usize] = "O".to_owned();
    }

    return map;
}

fn shift_rock(
    initial_rock_pos: (isize, isize),
    map: &Vec<Vec<String>>,
    direction: (isize, isize),
) -> (isize, isize) {
    match direction {
        NORTH => {
            if initial_rock_pos.0 == 0 {
                return initial_rock_pos;
            }
        }
        SOUTH => {
            if initial_rock_pos.0 == (map.len() - 1) as isize {
                return initial_rock_pos;
            }
        }
        WEST => {
            if initial_rock_pos.1 == 0 {
                return initial_rock_pos;
            }
        }
        EAST => {
            if initial_rock_pos.1 == (map[0].len() - 1) as isize {
                return initial_rock_pos;
            }
        }
        _ => panic!("Invalid direction passed"),
    }

    let mut pos = initial_rock_pos.clone();
    while next_pos_in_bounds(pos, map, direction) {
        let to_check = (
            (pos.0 + direction.0) as usize,
            (pos.1 + direction.1) as usize,
        );
        if &map[to_check.0][to_check.1] == "O" || &map[to_check.0][to_check.1] == "#" {
            break;
        } else {
            pos.0 += direction.0;
            pos.1 += direction.1;
        }
    }
    return pos;
}

fn next_pos_in_bounds(
    pos: (isize, isize),
    map: &Vec<Vec<String>>,
    direction: (isize, isize),
) -> bool {
    match direction {
        NORTH => pos.0 > 0,
        SOUTH => pos.0 < (map.len() - 1) as isize,
        WEST => pos.1 > 0,
        EAST => pos.1 < (map[0].len() - 1) as isize,
        _ => panic!("Invalid direction passed"),
    }
}

fn compare_rocks(r1: &(isize, isize), r2: &(isize, isize), direction: (isize, isize)) -> Ordering {
    match direction {
        NORTH => {
            if r1.0 > r2.0 {
                return Ordering::Greater;
            };
            if r1.0 < r2.0 {
                return Ordering::Less;
            };
            Ordering::Equal
        }
        SOUTH => {
            if r1.0 < r2.0 {
                return Ordering::Greater;
            };
            if r1.0 > r2.0 {
                return Ordering::Less;
            };
            Ordering::Equal
        }
        WEST => {
            if r1.1 > r2.1 {
                return Ordering::Greater;
            };
            if r1.1 < r2.1 {
                return Ordering::Less;
            };
            Ordering::Equal
        }
        EAST => {
            if r1.1 < r2.1 {
                return Ordering::Greater;
            };
            if r1.1 > r2.1 {
                return Ordering::Less;
            };
            Ordering::Equal
        }
        _ => panic!("Invalid direction passed"),
    }
}

fn calculate_load(map: &Vec<Vec<String>>) -> u32 {
    let mut load: u32 = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == "O" {
                load += (map.len() - row) as u32;
            }
        }
    }
    return load;
}
