use itertools::Itertools;

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

/// Day 10, Part 1
///
/// You ride your hang glider from Desert Island all the way up to
/// the floating metal island. While wandering around, you don't
/// find anything but you do find signs labeled "Hot Springs" and
/// quickly soon after realize that the ground is covered in a 2D
/// grid of pipes (the puzzle input).
///
/// An animal scurries into one of the pipes and spooks you. If you
/// want to get ahead of it, you should find the tile in the loop
/// that is furthest away from it along the loop of pipe that its in.
///
/// How many steps along the loop does it take to get from the
/// starting position to the point furthest away from the starting
/// position along the main pipe?
pub fn find_furthest_loop_section(pipe_map: &Vec<&str>) -> isize {
    let mut padded_map = pipe_map.iter().map(|s| format!(".{}.", s)).collect_vec();
    padded_map.insert(0, vec!["."; padded_map[0].len()].join(""));
    padded_map.push(vec!["."; padded_map[0].len()].join(""));
    let pipe_arr = padded_map
        .iter()
        .map(|s| s.split("").filter(|s| s != &"").collect_vec())
        .collect_vec();

    pipe_arr.iter().for_each(|v| println!("{}", v.join("")));
    let start_pos = find_start_position(&pipe_arr);
    let starting_pipe = Pipe {
        position: start_pos,
        connections: find_start_connections(start_pos, &pipe_arr),
    };

    let mut current = starting_pipe.build_positions()[0];
    let mut prev = starting_pipe;
    let mut pipe_pieces = 1;

    while current != start_pos {
        match Pipe::new(current, pipe_arr[current.0 as usize][current.1 as usize]) {
            Some(pipe) => {
                pipe_pieces += 1;
                let temp = *pipe
                    .build_positions()
                    .iter()
                    .find(|p| p != &&pipe.position && p != &&prev.position)
                    .unwrap();
                prev = pipe;
                current = temp;
            }
            None => panic!(
                "Unable to parse pipe {:?} with value {}",
                current, pipe_arr[current.0 as usize][current.1 as usize]
            ),
        }
    }

    return pipe_pieces / 2;
}

type Connection = (isize, isize);

#[derive(Clone, Debug)]
struct Pipe {
    position: (isize, isize),
    connections: (Connection, Connection),
}

impl Pipe {
    pub fn new(position: (isize, isize), value: &str) -> Option<Pipe> {
        let connections;
        match value {
            "|" => connections = (UP, DOWN),
            "-" => connections = (LEFT, RIGHT),
            "L" => connections = (UP, RIGHT),
            "J" => connections = (UP, LEFT),
            "7" => connections = (LEFT, DOWN),
            "F" => connections = (RIGHT, DOWN),
            "." => return None,
            _ => panic!("Pipe symbol unaccounted for: {}", value),
        }

        return Some(Pipe {
            position,
            connections,
        });
    }

    pub fn build_positions(&self) -> Vec<Connection> {
        return vec![
            (
                self.position.0 + self.connections.0 .0,
                self.position.1 + self.connections.0 .1,
            ),
            (
                self.position.0 + self.connections.1 .0,
                self.position.1 + self.connections.1 .1,
            ),
        ];
    }
}

// Helper functions
fn find_start_position(pipe_map: &Vec<Vec<&str>>) -> (isize, isize) {
    for i in 0..pipe_map.len() {
        for j in 0..pipe_map[0].len() {
            if pipe_map[i][j] == "S" {
                return (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }
    panic!("No starting location found.");
}

fn find_start_connections(
    start_pos: (isize, isize),
    pipe_map: &Vec<Vec<&str>>,
) -> (Connection, Connection) {
    let binding = vec![UP, DOWN, LEFT, RIGHT];
    let ok_dirs = binding
        .iter()
        .filter(|dir| {
            let target = (start_pos.0 + dir.0, start_pos.1 + dir.1);
            let value = pipe_map[target.0 as usize][target.1 as usize];
            match Pipe::new(target, &value.to_string()) {
                Some(pipe) => pipe.build_positions().contains(&start_pos),
                None => false,
            }
        })
        .collect_vec();
    return (*ok_dirs[0], *ok_dirs[1]);
}
