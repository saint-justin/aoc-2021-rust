#[allow(unused)]
/// Day 2, Part 1 -- https://adventofcode.com/2021/day/2
///
/// You need to figure out how to pilot the submarine!
/// Your horizontal position and depth both start at 0, your
/// input is a list of movement types that your submarine can
/// execute (examples below). What do you get if you multiply
/// your final horizontal position by your final depth?
///
/// "forward 5" -> x position += 5
/// "down 3"    -> y position += 3
/// "up 6"      -> y position -= 6
pub fn find_positional_product(logs_input: &Vec<&str>) -> u32 {
    let mut x_pos = 0;
    let mut y_pos = 0;

    for item in logs_input {
        let args: Vec<&str> = item.split(" ").collect();
        let amount = args[1].parse::<u32>().unwrap();
        match args[0] {
            "forward" => x_pos += amount,
            "down" => y_pos += amount,
            "up" => y_pos -= amount,
            _ => panic!("Invalid command type!"),
        }
    }

    x_pos * y_pos
}

#[test]
fn check_position_depth_product() {
    let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let actual = find_positional_product(&input);
    let expected: u32 = 150; // x_pos 15 * y_pos 10 => 150
    assert_eq!(actual, expected);
}

/// Day 2, Part 2
///
/// Based on some new calculations, it turns out the logs we're
/// reading from actually mean something totally different. We
/// should be tracking a 3rd value, aim, which starts at 0. The
/// commands for up and down change aim and forward actually does
/// two separate things (see examples below). With this new system,
/// what is the product of our final x and y positions?
///
/// "down 3"    -> aim += 3
/// "up 6"      -> aim -= 6
/// "forward 5" -> x position += 5
///             -> y position += aim*5

pub fn find_aimed_product(logs_input: &Vec<&str>) -> u32 {
    let mut aim = 0;
    let mut x_pos = 0;
    let mut y_pos = 0;

    for item in logs_input {
        let args: Vec<&str> = item.split(" ").collect();
        let amount = args[1].parse::<u32>().unwrap();
        match args[0] {
            "forward" => {
                x_pos += amount;
                y_pos += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Invalid command type!"),
        }
    }

    x_pos * y_pos
}

#[test]
fn check_aimed_position_depth_product() {
    let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let actual = find_aimed_product(&input);
    let expected: u32 = 900; // x_pos 15 * y_pos 10 => 150
    assert_eq!(actual, expected);
}
