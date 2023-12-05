use regex::Regex;

/// Day 5, Part 1 -- https://adventofcode.com/2023/day/5
/// 
/// This one is a doozy so I'd recommend reading the original first.
/// 
/// It turns out there's plenty of water for making snow on island island,
/// we just need sand to filter it. We can get sand from the gardener if
/// we help them with their food production problem. They have an almanac 
/// of seeds and how to process them (the puzzle input). Each entry in 
/// the almanac is one pieces of the seed processing workflow (seed-to-soil,
/// soil-to-fertilizer, ...and finally humidity-to-location). The almanac
/// contains a series of ranges representing the types of seeds that are the
/// input for each stage and what the expected output for that input should 
/// be. Any inputs not included within the seed range inputs stay the same.
/// For example:
/// 
/// seed-to-soil map:
/// 50 98 2
/// 
/// This almanac entry instructs us to take an input starting at 98 of length 2
/// (98 to 99 inclusive) and process any seed from that input and match it to
/// an output range starting from 50 (50 to 51 inclusive). This means that if
/// we had input seeds [98, 99, 99, 100], the outputs would be [50, 51, 51, 100]. 
/// The third seed does not change as it's not included in the input range.
/// 
/// What is the lowest location number that corresponds to any of the initial seeds?
pub fn find_lowest_initial_seed_location(almanac: &Vec<&str>) -> i64 {
    let re = Regex::new("([0-9]{1,})").unwrap();
    let seed_inputs = re.find_iter(almanac[0])
        .map(|n| n.as_str().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let almanac_pages = parse_entries(almanac, &re);
    return seed_inputs.iter().map(|seed| {
        let mut seed_value = *seed;
        for page in &almanac_pages {
            seed_value = update_by_almanac_page(seed_value, &page);
        }
        return seed_value;
    }).min().unwrap()
}

fn update_by_almanac_page(seed: i64, map: &Vec<SeedRange>) -> i64 {
    for range in map {
        let updated_value = range.process_seed(seed);
        if updated_value != seed {
            return updated_value;
        }
    }
    return seed
}

#[derive(Debug)]
struct SeedRange {
    input: i64,
    output: i64,
    range: i64,
}

impl SeedRange {
    pub fn process_seed(&self, seed: i64) -> i64 {
        if seed >= self.input && seed <= self.input + self.range {
            return self.output + (seed - self.input).abs() // output = output + diff
        }
        return seed;
    }
}

fn parse_entries(almanac: &Vec<&str>, re: &Regex) -> Vec<Vec<SeedRange>> {
    let mut entries: Vec<Vec<SeedRange>> = Vec::new();
    let mut temp: Vec<SeedRange> = Vec::new();

    for i in 2..almanac.len() {
        let line_split = almanac[i].split(" ").filter(|s| s != &"").collect::<Vec<&str>>();
        match line_split.len() {
            0 => {
                entries.push(temp);
                temp = Vec::new();
            }
            3 => {
                let parts = parse_nums(almanac[i], &re);
                temp.push(SeedRange { input: parts[1], output: parts[0], range: parts[2] });
            },
            _ => continue

        }
    }
    entries.push(temp);

    return entries;
}

// helpers
fn parse_nums(s: &str, re: &Regex) -> Vec<i64> {
    re.find_iter(s).filter_map(|digits| digits.as_str().parse().ok()).collect()
}