/// Day 2, Part 1 -- https://adventofcode.com/2023/day/2
///
/// You're playing a game with an elf on snow island (there's
/// no snow in sight). The elf pulls out a series of colored
/// cubes that are red, green, or blue (example logs below).
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
///
/// Your input is a log of collection of a few rounds per game
/// that you play with him and a count of R/G/B cubes he shows
/// you before putting them back into the bag. How many of the
/// games would've been possible to have been played if the
/// bag only contained 12 red, 13 green, and 14 blue cubes?
/// Return the sum of possible game ID's.
pub fn possible_game_id_sum(game_data: &Vec<&str>) -> u32 {
  return game_data
    .iter()
    .map(|game_data| parse_game(game_data))
    .filter(|game| game.is_valid())
    .map(|game| game.id)
    .sum();
}

/// Day 2, Part 2
///
/// While walking alongside the elf, they have you consider what
/// the fewest number of cubes required to play each given game
/// would have been. For example, in the game below it's 4 red,
/// 2 green, and 6 blue cubes.
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
///
/// The "power" of a set of cubes is equal to the product of the
/// minimum red, green, and blue cubes for a game. For the game
/// above, it would be 4 * 2 * 6 = 48. What is the sum of the
/// power of all the games?
pub fn find_power_sum(game_data: &Vec<&str>) -> u32 {
  return game_data
    .iter()
    .map(|game_data| parse_game(game_data))
    .map(|game| game.calculate_game_power())
    .sum();
}

// Helper to parse a game from a string
fn parse_game(game_data: &str) -> Game {
  let game_split = game_data.split(": ").collect::<Vec<&str>>();
  let game_id = game_split[0].split(" ").collect::<Vec<&str>>()[1];
  let round_data = game_split[1].split("; ").collect::<Vec<&str>>();
  let mut game = Game::new(game_id.parse::<u32>().unwrap());

  round_data.iter().for_each(|round| {
    let mut round_data = Round::new();
    round.split(", ").for_each(|color| {
      let color_split = color.split(" ").collect::<Vec<&str>>();
      round_data.update_color(color_split[1], color_split[0].parse::<u32>().unwrap());
    });
    game.rounds.push(round_data);
  });

  return game;
}

#[derive(Debug)]
struct Game {
  id: u32,
  rounds: Vec<Round>,
}
impl Game {
  pub fn new(id: u32) -> Game {
    Game {
      id,
      rounds: Vec::new(),
    }
  }

  pub fn is_valid(&self) -> bool {
    self.rounds.iter().all(|round| round.is_valid())
  }

  pub fn calculate_game_power(&self) -> u32 {
    let r = self.rounds.iter().map(|r| r.red).max().unwrap();
    let g = self.rounds.iter().map(|r| r.green).max().unwrap();
    let b = self.rounds.iter().map(|r| r.blue).max().unwrap();

    r * g * b
  }
}

#[derive(Debug)]
struct Round {
  red: u32,
  green: u32,
  blue: u32,
}
impl Round {
  pub fn new() -> Round {
    Round {
      red: 0,
      green: 0,
      blue: 0,
    }
  }

  pub fn update_color(&mut self, color: &str, value: u32) {
    match color {
      "red" => self.red = value,
      "green" => self.green = value,
      "blue" => self.blue = value,
      _ => panic!("Invalid color passed! [{}]", color),
    }
  }

  pub fn is_valid(&self) -> bool {
    self.red <= 12 && self.green <= 13 && self.blue <= 14
  }
}
