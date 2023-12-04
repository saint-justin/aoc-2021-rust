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
    let vec_schematic: Vec<Vec<&str>> = schematic.iter()
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
            if is_number(schematic_row[i])  {
                temp_part.0.push_str(schematic_row[i]);
                if temp_part.1 == 0 { temp_part.1 = i }
            } else if temp_part.0 != "".to_owned() {
                scraped_parts.push(temp_part.clone());
                temp_part = ("".to_owned(), 0);
            }
        }
        if temp_part.0 != "".to_owned() { scraped_parts.push(temp_part) }

        return scraped_parts
    }

    // helper that checks surrounding area for symbols
    fn neighbors_contain_symbol(row: usize, col: &usize, width: usize, schematic: &Vec<Vec<&str>>) -> bool {
        for i in row-1 ..= row+1 {
            for j in col-1 ..= col+width {
                if is_symbol(schematic[i][j]) {
                    return true;
                }
            }
        }

        return false;
    }
    fn is_symbol(char_str: &str) -> bool { !"0123456789.".contains(char_str) }
    fn is_number(char_str: &str) -> bool { "0123456789".contains(char_str) }

/*
fn parse_parts(schematic: &Vec<&str>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let re_parts = Regex::new(r"([0-9]{1,})").unwrap();
    let re_symbols = Regex::new(r"([^0-9.\n\r])").unwrap();
    
    for (row_index, row) in schematic.iter().enumerate() {
        for (_, [part_id_str]) in re_parts.captures_iter(row).map(|c| c.extract()) {
            parts.push(Part {
                id: part_id_str.to_owned(),
                is_valid: check_validity(part_id_str, row_index, schematic, &re_symbols),
            })
        }
    }

    println!("{:?}", parts.iter().map(|p| p.id.to_owned()).collect::<Vec<String>>());

    return parts;
}

fn check_validity(part_id: &str, row: usize, schematic: &Vec<&str>, re: &Regex) -> bool {
    let mut out = "".to_owned();
    let col = schematic[row].find(part_id).unwrap();
    let left_valid = col > 0;
    let right_valid = col + part_id.len() < schematic[0].len();
    println!("Symbol ID                   [{}]", part_id);

    // top row
    if row > 0 {
        if left_valid { out.push_str(&schematic[row-1][col-1..col]) }
        out.push_str(&schematic[row-1][col..col+part_id.len()]);
        if right_valid { out.push_str(&schematic[row-1][col+part_id.len()..col+part_id.len()+1]) }
    }

    // center row
    if left_valid { out.push_str(&schematic[row][col-1..col]) }
    if right_valid { out.push_str(&schematic[row][col+part_id.len()..col+part_id.len()+1]) }

    // bottom row
    if row < schematic.len() - 1 {
        if left_valid { out.push_str(&schematic[row+1][col-1..col]) }
        out.push_str(&schematic[row+1][col..col+part_id.len()]);
        if right_valid { out.push_str(&schematic[row+1][col+part_id.len()..col+part_id.len()+1]) }
    }

    return re.find(&out).is_some()
}

#[derive(Debug)]
struct Part {
    id: String,
    is_valid: bool,
}

#[test]
pub fn default_check_validity() {
    let s = "467..114..
                   ...&......
                   ..35..633.
                   ......#...
                   617/......
                   .....+.58.
                   ..592.....
                   ......755.
                   ...$.=....
                   .664.598..";

    assert_eq!(valid_parts_sum(&s.split(['\n']).map(|e| e.trim()).collect()), 4361);
}

#[test]
pub fn check_validity_surrounding_all() {
    let tl = "*..
                    .3.
                    ...";

    let tc = ".*.
                    .3.
                    ...";

    let tr = "..*
                    .3.
                    ...";

    let ml = "...
                    *3.
                    ...";

    let mr = "...
                    .3*
                    ...";

    let bl = "...
                    .3.
                    *..";

    let bm = "...
                    .3.
                    .*.";

    let br = "...
                    .3.
                    ..*";

    assert_eq!(valid_parts_sum(&tl.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tc.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tr.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&ml.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&mr.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&bl.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&bm.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&br.split(['\n']).map(|e| e.trim()).collect()), 3);
}

#[test]
pub fn check_validity_corners() {
    let tl1 = "3*.
                    ..
                    ...";

    let tl2 = "3..
                    .*.
                    ...";

    let tl3 = "3..
                     *..
                     ...";

    let tr1 = ".*3
                     ...
                     ...";
 
    let tr2 = "..3
                     .*.
                     ...";
 
    let tr3 = "..3
                     ..*
                     ...";

    let bl1 = "...
                     *..
                     3..";
 
    let bl2 = "...
                     .*.
                     3..";
 
    let bl3 = "...
                     ...
                     3*.";

    let br1 = "...
                     ..*
                     ..3";
 
    let br2 = "...
                     .*.
                     ..3";
 
    let br3 = "...
                     ...
                     .*3";

    let custom = "....*..
                        ...152.";


    assert_eq!(valid_parts_sum(&tl1.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tl2.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tl3.split(['\n']).map(|e| e.trim()).collect()), 3);

    assert_eq!(valid_parts_sum(&tr1.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tr2.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&tr3.split(['\n']).map(|e| e.trim()).collect()), 3);

    assert_eq!(valid_parts_sum(&bl1.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&bl2.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&bl3.split(['\n']).map(|e| e.trim()).collect()), 3);

    assert_eq!(valid_parts_sum(&br1.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&br2.split(['\n']).map(|e| e.trim()).collect()), 3);
    assert_eq!(valid_parts_sum(&br3.split(['\n']).map(|e| e.trim()).collect()), 3);

    assert_eq!(valid_parts_sum(&custom.split(['\n']).map(|e| e.trim()).collect()), 152);
}
*/