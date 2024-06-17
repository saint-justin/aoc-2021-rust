#[derive(Debug)]
struct PasswordValidation {
  n1: usize,
  n2: usize,
  character: String,
  password: String,
}

/// Day 2, Part 1 -- https://adventofcode.com/2020/day/2
///
/// You need to rent a toboggan to get to your airport! The rental
/// company is having issues with their password database, it seems
/// to be corrupted. To debug the problem, the rental company
/// created a list of passwords matched with the corporate policy
/// that was live when that password was set, example below.
///
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
///
/// Each line gives the policy then the password. The policy
/// indicates the fewest and most times a given letter must appear
/// to be considered valid. For the example, [1-3 a] means the
/// password must contain "a" at least 1 time and at most 3 times.
///
/// To determine the severity of the database corruption,
/// how many of the passwords are valid according to their policies?
pub fn find_valid_passwords_by_count(password_inputs: &Vec<&str>) -> usize {
  let pwvalidation_objects: Vec<PasswordValidation> = password_inputs
    .into_iter()
    .map(|str| parse_string_to_pwvalidation(str))
    .collect();

  let valid_passwords = pwvalidation_objects.into_iter().filter(|pwval| {
    let target_char_count = pwval.password.matches(&pwval.character).count();
    return target_char_count <= pwval.n2 && target_char_count >= pwval.n1;
  });

  return valid_passwords.count();
}

/// Day 2, Part 2
///
/// Oh wait, the shopkeeper at the toboggan shop realizes the password
/// policy actually worked a little differently. Instead of checking the
/// count of the target character, we should instead be checking that
/// the target character is in one of the two listed positions instead.
///
/// According to the new system, the demo input given in part one would only
/// yield one valid password, the 3rd password is now invalid because
/// both positions 2 and 9 (indices 0 and 8) contain the character "c".
///
/// How many passwords are valid given the new interpretation of the policy?
pub fn find_valid_passwords_by_position(password_inputs: &Vec<&str>) -> usize {
  let pwvalidation_objects: Vec<PasswordValidation> = password_inputs
    .into_iter()
    .map(|str| parse_string_to_pwvalidation(str))
    .collect();

  let valid_passwords = pwvalidation_objects
    .into_iter()
    .filter(|pwval| password_valid_by_position(&pwval));

  return valid_passwords.count();
}

// Helper to parse string into a more manageable struct
fn parse_string_to_pwvalidation(input_str: &str) -> PasswordValidation {
  let input_split: Vec<&str> = input_str.split(" ").collect();
  let min_max_split: Vec<&str> = input_split[0].split("-").collect();

  PasswordValidation {
    n1: min_max_split[0].parse::<usize>().unwrap(),
    n2: min_max_split[1].parse::<usize>().unwrap(),
    character: input_split[1][0..1].to_owned(),
    password: input_split[2].to_owned(),
  }
}

// Helper to determine if a password is valid based on its position
fn password_valid_by_position(pwval: &PasswordValidation) -> bool {
  let char_1 = pwval.password.chars().nth(pwval.n1 - 1);
  let char_2 = pwval.password.chars().nth(pwval.n2 - 1);
  let target_char: char = pwval.character.chars().collect::<Vec<char>>()[0];

  if char_1.is_none() && char_2.is_none() {
    return false;
  } else if char_1.is_some() && char_2.is_some() {
    return (char_1.unwrap() == target_char) ^ (char_2.unwrap() == target_char);
  } else if char_1.is_some() {
    return char_1.unwrap() == target_char;
  } else {
    return char_2.unwrap() == target_char;
  }
}
