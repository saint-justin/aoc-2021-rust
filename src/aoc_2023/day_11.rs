use itertools::Itertools;

/// Day 11, Part 1 -- https://adventofcode.com/2023/day/11
///
/// You follow the signs for "Hot Springs" and eventually come across
/// an abservatory where an Elf researcher is finishing some work. He
/// doesn't know anything about missing machine parts, he's just
/// visiting to finish this research, but if you help him finish his
/// research he can take you straight to the hot springs where you
/// may find someone who does know about the missing parts.
///
/// The researcher is trying to find the shortest path between every
/// pair of galaxies that he has happed so far. The galaxies are
/// given in a 2D grid and distance is measured as Manhattan Distance.
/// The researcher also clarifies that any row or column that has no
/// stars in it ("#") should be treated as 2 empty spaces and not just
/// one.
///
/// What is the sum of the lengths between every pair of galaxies?  
pub fn find_distance_sum(unexpanded_star_map: &Vec<&str>) -> u32 {
  let star_map = expand_star_map(unexpanded_star_map);
  let (h_gaps, v_gaps) = find_gaps(&star_map);
  let stars: Vec<(i32, i32)> = find_stars(&star_map);

  let mut total_distance = 0;
  for i in 0..stars.len() - 1 {
    for j in i + 1..stars.len() {
      let x_gaps = count_gaps_in_range(&h_gaps, stars[i].0, stars[j].0);
      let x_diff = (stars[i].0 - stars[j].0).abs() - x_gaps;
      let y_gaps = count_gaps_in_range(&v_gaps, stars[i].1, stars[j].1);
      let y_diff = (stars[i].1 - stars[j].1).abs() - y_gaps;
      total_distance += (x_diff + y_diff + x_gaps * 2 + y_gaps * 2) as u32;
    }
  }

  return total_distance;
}

/// Day 11, Part 2
///
/// It turns out galaxies are waaaaay older and as a result waaaaay
/// further apart than initially anticipated. Instead of empty columns
/// being 2 columns instead of 1, they should instead be approximated
/// to 1m columns instead of 1.
///
/// What's the sum of the new lengths between stars at scale?
pub fn find_scaled_distance_sum(unexpanded_star_map: &Vec<&str>) -> u64 {
  let star_map = expand_star_map(unexpanded_star_map);
  let (h_gaps, v_gaps) = find_gaps(&star_map);
  let stars: Vec<(i32, i32)> = find_stars(&star_map);

  let mut total_distance: u64 = 0;
  for i in 0..stars.len() - 1 {
    for j in i + 1..stars.len() {
      let x_gaps = count_gaps_in_range(&h_gaps, stars[i].0, stars[j].0);
      let x_diff = (stars[i].0 - stars[j].0).abs() - x_gaps;
      let y_gaps = count_gaps_in_range(&v_gaps, stars[i].1, stars[j].1);
      let y_diff = (stars[i].1 - stars[j].1).abs() - y_gaps;
      total_distance += (x_diff + y_diff + x_gaps * 1_000_000 + y_gaps * 1_000_000) as u64;
    }
  }

  return total_distance;
}

// helper fns
fn expand_star_map(unexpanded_star_map: &Vec<&str>) -> Vec<Vec<String>> {
  unexpanded_star_map
    .iter()
    .map(|s| {
      s.split("")
        .filter(|s| s != &"")
        .map(|s| s.to_owned())
        .collect_vec()
    })
    .collect_vec()
}

fn find_gaps(star_map: &Vec<Vec<String>>) -> (Vec<i32>, Vec<i32>) {
  let v_gaps = star_map
    .iter()
    .enumerate()
    .filter(|(_, row)| !row.contains(&"#".to_owned()))
    .map(|(i, _)| i as i32)
    .collect_vec();

  let mut h_gaps: Vec<i32> = Vec::new();
  for i in 0..star_map[0].len() {
    if !star_map.iter().map(|row| row[i].clone()).any(|s| s == "#") {
      h_gaps.push(i as i32);
    }
  }

  return (v_gaps, h_gaps);
}

fn find_stars(star_map: &Vec<Vec<String>>) -> Vec<(i32, i32)> {
  let mut stars = Vec::new();
  for i in 0..star_map.len() {
    for j in 0..star_map[0].len() {
      if star_map[i][j] == "#".to_owned() {
        stars.push((i as i32, j as i32));
      }
    }
  }
  return stars;
}

fn count_gaps_in_range(gaps: &Vec<i32>, n1: i32, n2: i32) -> i32 {
  let range = if n1 < n2 { n1..n2 } else { n2..n1 };
  return gaps.iter().filter(|n| range.contains(n)).count() as i32;
}
