/// Day 3, Part 1 -- https://adventofcode.com/2020/day/3
///
/// You successfully rented your toboggan! Now, you just
/// need to chart a path to ride it safely to the airtport.
/// You have a map of the area (puzzle input) that infinitely
/// tiles horizontally.
///
/// If you started in the top-left corner of the map with
/// a slope of right 3 down 1, how many trees would you encounter?
pub fn count_tree_collisions(toboggan_terrain: &Vec<&str>) -> u64 {
  let split_terrain = toboggan_terrain
    .into_iter()
    .map(|line| {
      line
        .split("")
        .filter(|str| str != &"") // trim didn't work for some reason
        .collect()
    })
    .collect::<Vec<Vec<&str>>>();

  return count_collisions_at_slope(3, 1, &split_terrain);
}

/// Day 3, Part 2
///
/// We know how many trees we would hit! Maybe now, though, instead
/// of just finding how many trees we will hit, we instead try out
/// all the slopes that we can launch at and choose the least damaging.
///
/// According to the launch platform, the slopes we can launch at include:
/// Right 1, Down 1
/// Right 3, Down 1
/// Right 5, Down 1
/// Right 7, Down 1
/// Right 1, Down 2
///
/// What do we get if we multiply together the count of trees encountered
/// on each of the potential slopes?
pub fn count_tree_collision_product(toboggan_terrain: &Vec<&str>) -> u64 {
  let split_terrain = toboggan_terrain
    .into_iter()
    .map(|line| {
      line
        .split("")
        .filter(|str| str != &"") // trim didn't work for some reason
        .collect()
    })
    .collect::<Vec<Vec<&str>>>();

  let collisions_per_slope = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
    .into_iter()
    .map(|slope| count_collisions_at_slope(slope[0], slope[1], &split_terrain))
    .collect::<Vec<u64>>();

  return collisions_per_slope.iter().product();
}

// Helper to find the amount of collisions at a given slope for a given terrain
// Note: u64 is required for calculating the collision count product later
fn count_collisions_at_slope(slope_x: usize, slope_y: usize, terrain: &Vec<Vec<&str>>) -> u64 {
  let mut x_pos = 0;
  let mut y_pos = 0;
  let x_reset = terrain[0].len();
  let mut collisions = 0;
  while y_pos + slope_y < terrain.len() {
    y_pos += slope_y;
    x_pos += slope_x;
    x_pos %= x_reset;
    if terrain[y_pos][x_pos] == "#" {
      collisions += 1;
    }
  }

  // println!("Individual answer! [{}, {}] -> {:3} collisions", slope_x, slope_y, collisions);

  return collisions;
}
