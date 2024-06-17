use itertools::Itertools;
use regex::Regex;

/// Day 6, Part 1 -- https://adventofcode.com/2023/day/6
///
/// You find your way back to Island Island but you don't find the sand
/// that you need for filtration. While searching around, you spot a boating
/// competition where the winner is sent to Desert Island! You quickly enter,
/// only to realize that you'll have to beat ever single record (puzzle input)
/// in order to be the winner. It also turns out, the boats are toy boats.
/// By holding down the button on top of the toy boat, you add speed to it.
/// One second held makes it run 1mm p/s faster. To settle your nerves, you
/// need to figure out your margin of error for winning the competition.
///
/// Time:      7  15   30
/// Distance:  9  40  200
///
/// In the example above, there are 3 races where the current winners had
/// their boats cover distances 9mm, 40mm, and 200m in 7s, 15s, and 30s,
/// respectively. For the first race (7s/9mm), the only ways to win are
/// by holding down the button for 2, 3, 4, or 5 seconds. Any shorter or
/// any longer and the boat doesn't make it across the finish.
///
/// In total, for the first race there are 4 options, for the 2nd race there
/// are 8 options, and for the 3rd race there are 9 options. That means
/// your total options to win, and the puzzle output, would be 4 * 8 * 9.
/// How many ways are there to win in your input?
pub fn find_multisolution_product(record_sheet: &Vec<&str>) -> u64 {
  let re = Regex::new("([0-9]{1,})").unwrap();
  let times_and_distances = record_sheet
    .iter()
    .map(|s| {
      return re
        .find_iter(s)
        .map(|n| n.as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    })
    .collect::<Vec<Vec<u64>>>();

  let mut valid_race_opts: Vec<u64> = Vec::new();
  for i in 0..times_and_distances[0].len() {
    let min = find_min_valid_time(times_and_distances[0][i], times_and_distances[1][i]);
    let max = find_max_valid_time(times_and_distances[0][i], times_and_distances[1][i]);
    valid_race_opts.push(max - min + 1);
  }
  valid_race_opts.iter().product()
}

/// Day 6, Part 2
///
/// Oh no, it turns out there's only one race, the input just had some
/// kerning issues!
///
/// Time:      7  15   30
/// Distance:  9  40  200
///
/// Instead of there being three races which give you 7s, 15s, and 30s
/// to cover 9mm, 40mm, and 200mm, it's actually one HUGE race where you
/// have 71530s to cover 940200mm. Given this new info, how many ways
/// are there for you to win this new race?
pub fn find_solution_large_input(record_sheet: &Vec<&str>) -> u64 {
  let re = Regex::new("([0-9]{1,})").unwrap();
  let time_distance = record_sheet
    .iter()
    .map(|s| {
      re.find_iter(s)
        .map(|n_str| n_str.as_str().to_owned())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap()
    })
    .collect_vec();
  let min = find_min_valid_time(time_distance[0], time_distance[1]);
  let max = find_max_valid_time(time_distance[0], time_distance[1]);
  return max - min + 1; // adjusting for off-by-one because upper is inclusive
}

// Helpers
fn find_min_valid_time(time: u64, distance: u64) -> u64 {
  for windup in 0..time {
    let time_remaining = time - windup;
    if time_remaining * windup > distance {
      return windup;
    }
  }
  panic!("No valid min distance found");
}

fn find_max_valid_time(time: u64, distance: u64) -> u64 {
  for windup_time in (0..time).rev() {
    let time_remaining = time - windup_time;
    if time_remaining * windup_time > distance {
      return windup_time;
    }
  }
  panic!("No valid min distance found");
}

#[test]
fn test_find_min_and_max() {
  assert_eq!(find_min_valid_time(7, 9), 2);
  assert_eq!(find_max_valid_time(7, 9), 5);
}

#[test]
fn test_find_large() {
  let input1 = "Time:      7
                             Distance:  9"
    .split(['\n'])
    .map(|e| e.trim())
    .collect();

  let input2 = "Time:      15
                             Distance:  40"
    .split(['\n'])
    .map(|e| e.trim())
    .collect();

  let input3 = "Time:      30
                             Distance:  200"
    .split(['\n'])
    .map(|e| e.trim())
    .collect();

  assert_eq!(find_solution_large_input(&input1), 4);
  assert_eq!(find_solution_large_input(&input2), 8);
  assert_eq!(find_solution_large_input(&input3), 9);
}
