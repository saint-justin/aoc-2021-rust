use regex::Regex;

/// Day 4, Part 1 -- https://adventofcode.com/2023/day/4
/// 
/// The elf at the top of the gondola says that an elf, the gardener, 
/// who is on another island would be more likely to know about the 
/// water source you've been looking for. He'd be happy to let you 
/// borrow his boat to get to the gardener but he wants a quick trade.
/// This elf has a bunch of scratchers but he needs to find out what 
/// he has won! 
/// 
/// Each scratcher is given in the format below. One win on a card
/// is worth one point and each subsequent win doubles that amount
/// (e.g 1, 2, 4, 8, 16...)-- How many points has the elf won?
/// 
///   id  :  winning nums  | selected nums
/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
pub fn calculate_scratcher_points(scratcher: &Vec<&str>) -> u32 {
  let re = Regex::new("([0-9]{1,})").unwrap();
  
  return scratcher.iter()
    .map(|line| {
      let scratcher_parts: Vec<&str> = line.split([':', '|']).collect();
      let winners: Vec<u32> = parse_nums(scratcher_parts[1], &re);
      let picks: Vec<u32> = parse_nums(scratcher_parts[2], &re);
      let found_winners = picks.iter().filter(|n| winners.contains(n)).count();
      if found_winners > 0 { return 2_u32.pow((found_winners - 1).try_into().unwrap())}
      else { return 0 }
    }).sum();
}

fn parse_nums(s: &str, re: &Regex) -> Vec<u32> {
  return re.find_iter(s)
    .filter_map(|digits| digits.as_str().parse().ok())
    .collect()
}