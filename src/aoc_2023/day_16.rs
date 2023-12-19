use std::collections::HashSet;

use itertools::Itertools;
const NORTH: Vector2D = (-1, 0);
const SOUTH: Vector2D = (1, 0);
const EAST: Vector2D = (0, 1);
const WEST: Vector2D = (0, -1);

/// Day 16, Part 1 -- https://adventofcode.com/2023/day/16
///
/// Prompt is very large, see markdown file:
/// src\aoc_2023\prompts\day_17.md
pub fn find_energized_tiles(floor_map: &Vec<&str>) -> usize {
    let map = floor_map
        .iter()
        .map(|s| {
            s.split("")
                .map(|s| s.to_owned())
                .filter(|s| s != "")
                .collect_vec()
        })
        .collect_vec();

    let pos = (0, 0);
    match map[pos.0 as usize][pos.1 as usize].as_str() {
        "." | "-" => return explore(vec![Beam::new(pos, EAST)], &map),
        r"\" | "|" => return explore(vec![Beam::new(pos, SOUTH)], &map),
        _ => panic!("Unaccounted for!"),
    }
}

/// Day 16, Part 2
pub fn find_max_energized_tiles(floor_map: &Vec<&str>) -> usize {
    let map = floor_map
        .iter()
        .map(|s| {
            s.split("")
                .map(|s| s.to_owned())
                .filter(|s| s != "")
                .collect_vec()
        })
        .collect_vec();

    let mut start_positions: Vec<Vector2D> = Vec::new();
    for i in 0..map[0].len() {
        start_positions.push((0, i as isize));
        start_positions.push((map.len() as isize - 1, i as isize));
    }

    let values = start_positions.iter().map(|pos| {
        if pos.0 == 0 {
            match map[pos.0 as usize][pos.1 as usize].as_str() {
                "." | "|" => return explore(vec![Beam::new(*pos, SOUTH)], &map),
                "/" => return explore(vec![Beam::new(*pos, WEST)], &map),
                r"\" => return explore(vec![Beam::new(*pos, EAST)], &map),
                "-" => return explore(vec![Beam::new(*pos, EAST), Beam::new(*pos, WEST)], &map),
                _ => panic!("Unaccounted for!"),
            }
        } else {
            match map[pos.0 as usize][pos.1 as usize].as_str() {
                "." | "|" => return explore(vec![Beam::new(*pos, NORTH)], &map),
                "/" => return explore(vec![Beam::new(*pos, EAST)], &map),
                r"\" => return explore(vec![Beam::new(*pos, WEST)], &map),
                "-" => return explore(vec![Beam::new(*pos, EAST), Beam::new(*pos, WEST)], &map),
                _ => panic!("Unaccounted for setup!"),
            }
        }
    });

    return values.max().unwrap();
}

fn explore(starting_arr: Vec<Beam>, passed_map: &Vec<Vec<String>>) -> usize {
    let mut explored: HashSet<(Vector2D, Vector2D)> = HashSet::new();
    let mut unexplored_beams: Vec<Beam> = starting_arr;
    let mut current = unexplored_beams[0];
    explored.insert((current.pos, current.dir));

    loop {
        let starting_pos = current.pos;

        if !next_pos_valid(current.pos, current.dir, passed_map) {
            unexplored_beams.remove(0);
            if unexplored_beams.len() == 0 {
                break;
            }
            current = unexplored_beams[0];
            continue;
        }

        if let Some(new_beam) = current.next(passed_map) {
            unexplored_beams.push(new_beam);
        }

        if starting_pos == current.pos || explored.contains(&(current.pos, current.dir)) {
            unexplored_beams.remove(0);
            if unexplored_beams.len() == 0 {
                break;
            }
            current = unexplored_beams[0];
            continue;
        }

        explored.insert((current.pos, current.dir));
    }

    explored
        .iter()
        .map(|ex| ex.0)
        .collect::<HashSet<Vector2D>>()
        .into_iter()
        .collect_vec()
        .len()
}

type Vector2D = (isize, isize);
fn v_sum(v1: Vector2D, v2: Vector2D) -> Vector2D {
    (v1.0 + v2.0, v1.1 + v2.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    pos: Vector2D,
    dir: Vector2D,
}

impl Beam {
    pub fn new(pos: Vector2D, dir: Vector2D) -> Beam {
        Beam { pos, dir }
    }

    pub fn next(&mut self, map: &Vec<Vec<String>>) -> Option<Beam> {
        let next_pos = v_sum(self.dir, self.pos);
        let next_ch = map[next_pos.0 as usize][next_pos.1 as usize].as_str();
        match (self.dir, next_ch) {
            (NORTH, ".") | (SOUTH, ".") | (EAST, ".") | (WEST, ".") => self.pos = next_pos,
            (NORTH, "|") | (SOUTH, "|") | (EAST, "-") | (WEST, "-") => self.pos = next_pos,

            (NORTH, "/") => {
                self.pos = next_pos;
                self.dir = EAST;
            }
            (NORTH, r"\") => {
                self.pos = next_pos;
                self.dir = WEST;
            }
            (NORTH, "-") => {
                self.pos = next_pos;
                self.dir = EAST;
                return Some(Beam::new(next_pos, WEST));
            }

            (SOUTH, "/") => {
                self.pos = next_pos;
                self.dir = WEST;
            }
            (SOUTH, r"\") => {
                self.pos = next_pos;
                self.dir = EAST;
            }
            (SOUTH, "-") => {
                self.pos = next_pos;
                self.dir = EAST;
                return Some(Beam::new(next_pos, WEST));
            }

            (EAST, "/") => {
                self.pos = next_pos;
                self.dir = NORTH;
            }
            (EAST, r"\") => {
                self.pos = next_pos;
                self.dir = SOUTH;
            }
            (EAST, "|") => {
                self.pos = next_pos;
                self.dir = NORTH;
                return Some(Beam::new(next_pos, SOUTH));
            }

            (WEST, "/") => {
                self.pos = next_pos;
                self.dir = SOUTH;
            }
            (WEST, r"\") => {
                self.pos = next_pos;
                self.dir = NORTH;
            }
            (WEST, "|") => {
                self.pos = next_pos;
                self.dir = NORTH;
                return Some(Beam::new(next_pos, SOUTH));
            }

            _ => println!(
                "Skipping beam state: {:?}  next_c: {}  next_p: {:?}",
                self, next_ch, next_pos
            ),
        }

        None
    }
}

pub fn next_pos_valid(pos: Vector2D, dir: Vector2D, map: &Vec<Vec<String>>) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as isize || pos.1 >= map[0].len() as isize {
        return false;
    }

    match dir {
        NORTH => pos.0 >= 1,
        SOUTH => pos.0 != map.len() as isize - 1,
        EAST => pos.1 != map[0].len() as isize - 1,
        WEST => pos.1 >= 1,
        _ => panic!("Invalid direction: {:?}", dir),
    }
}
