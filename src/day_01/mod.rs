#[allow(unused)]
pub mod solutions {
    /// Day 1, Part 1 -- https://adventofcode.com/2021/day/1
    ///
    /// You're in a submarine trying to track how quickly your
    /// depth increases. To do this, you need to count how many
    /// times the depth measurement increases from the previous
    /// measurement.
    ///
    /// Your puzzle input is a list of depth measurements ranging
    /// from 100 to 10,000
    pub fn count_measurement_increases(measures_input: &Vec<&str>) -> u32 {
        let measures: Vec<u32> = measures_input
            .into_iter()
            .map(|measure| measure.parse::<u32>().unwrap())
            .collect();

        let mut count = 0;
        for i in 1..measures.len() {
            if measures[i - 1] < measures[i] {
                count += 1;
            }
        }

        return count;
    }

    #[test]
    fn measures_count_increases() {
        let test_measures = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let actual = count_measurement_increases(&test_measures);
        let expected = 7;
        assert_eq!(actual, expected);
    }

    /// Day 1, Part 2
    ///
    /// Counting every individual measure isn't returning useful info,
    /// instead we need to count via a 3-measurement sliding window.
    /// Count how many times the depth measurement increases from the
    /// previous measurement within a 3-entry sliding window
    pub fn count_windowed_measurement_increases(measures_input: &Vec<&str>) -> u32 {
        let measures: Vec<u32> = measures_input
            .into_iter()
            .map(|measure| measure.parse::<u32>().unwrap())
            .collect();

        let mut count = 0;
        for i in 3..measures.len() {
            if measures[i - 3] < measures[i] {
                count += 1;
            }
        }

        return count;
    }

    #[test]
    fn measures_window_count_increases() {
        let test_measures = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let actual = count_windowed_measurement_increases(&test_measures);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
