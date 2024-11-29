pub fn find_frequency(frequency_list: &Vec<&str>) -> i32 {
  frequency_list.iter()
    .map(|n_as_str| n_as_str.parse::<i32>().unwrap())
    .sum()
}

