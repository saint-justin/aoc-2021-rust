/// Day 3, Part 1 -- https://adventofcode.com/2023/day/3
///
/// You have an engine schematic (a list of strings which
/// represent an image) and you need to parse out every valid
/// part number on that schematic. A valid part number is a
/// string of unbroken numbers on the same row which are
/// adjacent to any symbol (e.g. # or \ but not .). Diagonal
/// adjacency counts.
///
/// Return the sum of all valid parts on the schematic.
pub fn valid_parts_sum(schematic: &Vec<&str>) -> u32 {
  let vec_schematic: Vec<Vec<&str>> = schematic
    .iter()
    .map(|line| line.split("").filter(|ch| ch != &"").collect::<Vec<&str>>())
    .collect();

  let mut parts_sum = 0;
  for (row_index, line) in vec_schematic.iter().enumerate() {
    let row_parts = scrape_parts(line);
    for (part_id, col_index) in row_parts.iter() {
      if neighbors_contain_symbol(row_index, col_index, part_id.len(), &vec_schematic) {
        parts_sum += part_id.parse::<u32>().unwrap();
      }
    }
  }

  return parts_sum;
}

// Helper that takes the place of rust's broken regex system
fn scrape_parts(schematic_row: &Vec<&str>) -> Vec<(String, usize)> {
  let mut scraped_parts: Vec<(String, usize)> = Vec::new();
  let mut temp_part = ("".to_owned(), 0);
  for i in 0..schematic_row.len() {
    if is_number(schematic_row[i]) {
      temp_part.0.push_str(schematic_row[i]);
      if temp_part.1 == 0 {
        temp_part.1 = i
      }
    } else if temp_part.0 != "".to_owned() {
      scraped_parts.push(temp_part.clone());
      temp_part = ("".to_owned(), 0);
    }
  }
  if temp_part.0 != "".to_owned() {
    scraped_parts.push(temp_part)
  }

  return scraped_parts;
}

// helper that checks surrounding area for symbols
fn neighbors_contain_symbol(
  row: usize,
  col: &usize,
  width: usize,
  schematic: &Vec<Vec<&str>>,
) -> bool {
  for i in row - 1..=row + 1 {
    for j in col - 1..=col + width {
      if is_symbol(schematic[i][j]) {
        return true;
      }
    }
  }

  return false;
}
fn is_symbol(char_str: &str) -> bool {
  !"0123456789.".contains(char_str)
}
fn is_number(char_str: &str) -> bool {
  "0123456789".contains(char_str)
}
