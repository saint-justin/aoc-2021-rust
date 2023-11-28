/// Day 3, Part 1 -- https://adventofcode.com/2021/day/3
///
/// The submarine has been making some weird noises, so you generate
/// a diagnostic report just in case. The report is a list of binary
/// numbers that tell us useful info about the submarine. First, we're
/// checking our sub's power consumption.
///
/// We need to use the binary impout to find two new binary numbers, the
/// gamma rate and epsilon rate. Power consumption is the product of the
/// gamma and epsilon rates.
///
/// Each bit in the gamma rate can be determined by finding the most
/// common bit in the corresponding position of all numbers in the diagnostic
/// report (mode of all index 0's of the input, index 1's, index 2's, etc.)
/// The epsilon rate is calculated similarly, but using the least common
/// number instead of the most common number (gamma inverted).
///
/// What is the total power consumption of the ship.
pub fn get_submarine_power_consumption(log_inputs: &Vec<&str>) -> u32 {
    let mut value_sums: Vec<i32> = vec![0; log_inputs[0].len()];

    let split_logs: Vec<Vec<i32>> = log_inputs
        .into_iter()
        .map(|log_str| {
            return log_str
                .split("")
                .filter(|bit_str| bit_str != &"")
                .map(|bit_str| bit_str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        })
        .collect::<Vec<Vec<i32>>>();

    for log_index in 0..log_inputs.len() {
        for log_slot in 0..log_inputs[0].len() {
            value_sums[log_slot] += split_logs[log_index][log_slot];
        }
    }

    // These two chunks could likely just be stored as actual bits but this is just for practice
    let gamma_rate_bit_vec: Vec<&str> = value_sums
        .into_iter()
        .map(|v| v as usize > split_logs.len() / 2)
        .map(|b| if b { "1" } else { "0" })
        .collect();
    let epsilon_rate_bit_vec: Vec<&str> = gamma_rate_bit_vec
        .clone()
        .into_iter()
        .map(|bit_u32| if bit_u32 == "1" { "0" } else { "1" })
        .collect();

    let gamma_rate = u32::from_str_radix(&gamma_rate_bit_vec.join(""), 2).expect("Not binary!");
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_bit_vec.join(""), 2).expect("Not binary!");

    gamma_rate * epsilon_rate
}
