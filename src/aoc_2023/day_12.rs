use itertools::Itertools;
use regex::Regex;
use cached::proc_macro::cached;

/// Day 12, Part 1
/// 
/// You arrive at a the hot springs, but they're closed. Upon
/// entering the admin office, you'r enotified that there's 
/// actually a lava shortage which is preventing the hot springs
/// from being heated. Smoe of the springs are damaged and some
/// of them are operational, however the most recent report marks
/// a third option which is unknown. The engineer who produced the
/// report also included a description of contiguous groups of
/// damanged springs. 
/// 
/// #.#.### 1,1,3
/// 
/// An input will look like the line above, where "#" marks a damaged
/// spring, "." marks an operational spring, and the digits on the end
/// are the counts of contiguous damaged hot springs (in order). 
/// That said, most of the inputs will also include unknown as an option. 
/// 
/// ?###???????? 3,2,1
/// 
/// The input above shows that there's one unknown spring condition,
/// three damaged springs, and then 8 more unknown spring conditions. 
/// There are 10 possible arrangements of this spring that would 
/// describe every possible arrangement that would be valid according
/// to the info given.
/// 
/// What is the sum of all arrangements for every line of the input?
pub fn find_arrangement_sum (report: &Vec<&str>) -> u32 {
    let re_arrangement = Regex::new(r"([?.#]{1,})").unwrap();
    let re_broken = Regex::new(r"([#]{1,})").unwrap();
    let re_quantities = Regex::new(r"([0-9]{1,})").unwrap();
    let mut arrangement_sum: u32 = 0;


    for (i, line) in report.iter().enumerate() {
        // This solution is brute forced so giving user an output for completion percentage helps
        if i%10 == 0 { println!("{:.1}% complete ", (i as f32/report.len() as f32) * 100.0) };

        let arrangement = re_arrangement
            .find(line)
            .unwrap()
            .as_str();

        let quantities = re_quantities
            .find_iter(line)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect_vec();

        let unknown_indices = arrangement.split("")
            .filter(|s| s != &"")
            .enumerate()
            .filter(|(_, s)| s == &"?")
            .map(|(i, _)| i)
            .collect_vec();

        // splice in each filler and test if its valid
        let fillers = build_filler_iterations(unknown_indices.len());
        for filler in fillers {
            let mut new_arrangement = arrangement.to_owned();
            for (i, target) in unknown_indices.iter().enumerate() {
                let replacement_char = &filler.chars().nth(i).unwrap().to_string();
                new_arrangement.replace_range(target .. &(target+1), replacement_char)
            }
            if is_valid(&new_arrangement, &quantities, &re_broken) {
                arrangement_sum += 1;
            }
        }
    }

    return arrangement_sum;
}

// Helper fn to determine if a given arrangment matches its broken spring quantities
fn is_valid(arrangement: &str, quantities: &Vec<usize>, re: &Regex) -> bool {
    let arrangement_vec = re.find_iter(arrangement)
        .map(|m| m.as_str().len())
        .collect_vec();
    &arrangement_vec == quantities
}

// Helper that builds all potential fillers for size needed
#[cached]
fn build_filler_iterations(filler_count: usize) -> Vec<String> {
    let mut variations: Vec<String> = Vec::new();
    variations.push(".".to_owned());
    variations.push("#".to_owned());
    for _ in 1 .. filler_count {
        let mut new_variations: Vec<String> = Vec::new();
        for variation in variations {
            new_variations.push(format!("{}.", variation));
            new_variations.push(format!("{}#", variation));
        }
        variations = new_variations;
    }
    return variations;
}