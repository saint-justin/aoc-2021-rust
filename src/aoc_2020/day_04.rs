/// Day 4, Part 1 -- https://adventofcode.com/2020/day/4
///
/// You made it to the airport! However, you just realized that
/// instead of grabbing your passport, you grabbed your North Pole
/// Credentials instead. There's a very long line formed at the
/// automatic passport scanners which are moving slowly because
/// they're struggling to detect which passports have all their required
/// fields. The expected fields include byr, iyr, eyr, hgt, hcl, ecl,
/// pid, and cid which are all presented as key:value pairs in the
/// puzzle input.
///
/// All fields are required except for the cid field. How many passports
/// given in the puzzle input are valid?
pub fn count_valid_passports(logs_input: &Vec<&str>) -> usize {
  return generate_passports_from_dump(logs_input)
    .into_iter()
    .filter(|passport| passport.is_valid())
    .count();
}

/// Day 4, Part 2
///
/// Uh oh, your solution worked but some passports with invalid data
/// are starting to get through! Let's do some data validation on each
/// field on top of validating the fields exist. With the new validation
/// rules below, how many passports are valid?
///
/// byr - four digits; at least 1920 and at most 2002.
/// iyr - four digits; at least 2010 and at most 2020.
/// eyr - four digits; at least 2020 and at most 2030.
/// hgt - a number followed by either cm or in
///       If cm, the number must be at least 150 and at most 193.
///       If in, the number must be at least 59 and at most 76.
/// hcl - a # followed by exactly six characters 0-9 or a-f.
/// ecl - exactly one of: amb blu brn gry grn hzl oth.
/// pid - a nine-digit number, including leading zeroes.
/// cid - ignored, missing or not (still)
pub fn count_passports_validated_by_field(logs_input: &Vec<&str>) -> usize {
  return generate_passports_from_dump(logs_input)
    .into_iter()
    .filter(|passport| passport.is_valid_strict())
    .count();
}

fn generate_passports_from_dump(log_dump: &Vec<&str>) -> Vec<Passport> {
  let mut passport_dumps_collapsed: Vec<String> = vec![];
  let mut temp_passport = "".to_owned();
  for log in log_dump {
    if log.trim() == "" {
      passport_dumps_collapsed.push(temp_passport);
      temp_passport = "".to_owned();
      continue;
    } else {
      if temp_passport == "" {
        temp_passport = log.trim().to_owned().to_owned();
      } else {
        temp_passport = format!("{} {}", temp_passport, log.trim());
      }
    }
  }
  passport_dumps_collapsed.push(temp_passport);

  return passport_dumps_collapsed
    .into_iter()
    .map(|collapsed_passport_text| {
      let mut passport = Passport::new();
      for property in collapsed_passport_text.split(" ") {
        passport.add_property(property)
      }
      return passport;
    })
    .collect();
}

pub struct Passport {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>,
}

impl Passport {
  pub fn new() -> Passport {
    Passport {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None,
    }
  }

  pub fn is_valid(&self) -> bool {
    let properties: Vec<&Option<String>> = vec![
      &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid,
    ];
    return properties.iter().all(|property| property.is_some());
  }

  pub fn is_valid_strict(&self) -> bool {
    let properties: Vec<&Option<String>> = vec![
      &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid,
    ];
    if !properties.iter().all(|property| property.is_some()) {
      return false;
    }

    return vec![
      Self::validate_year(&self.byr, 1920, 2002),
      Self::validate_year(&self.iyr, 2010, 2020),
      Self::validate_year(&self.eyr, 2020, 2030),
    ]
    .iter()
    .all(|b| *b);
  }

  pub fn add_property(&mut self, new_property: &str) {
    let key_value_pair = new_property.split(":").collect::<Vec<&str>>();
    let value = key_value_pair[1].to_owned();
    match key_value_pair[0] {
      "byr" => self.byr = Some(value),
      "iyr" => self.iyr = Some(value),
      "eyr" => self.eyr = Some(value),
      "hgt" => self.hgt = Some(value),
      "hcl" => self.hcl = Some(value),
      "ecl" => self.ecl = Some(value),
      "pid" => self.pid = Some(value),
      "cid" => self.cid = Some(value),
      _ => panic!("Invalid property passed! [{}]", new_property),
    }
  }

  fn validate_year(yr: &Option<String>, min: u32, max: u32) -> bool {
    if yr.is_none() {
      return false;
    }
    if yr.as_ref().unwrap().len() != 4 {
      return false;
    }
    match yr.as_ref().unwrap().parse::<u32>() {
      Ok(n) => return n >= min && n <= max,
      Err(_) => return false,
    }
  }

  fn validate_height(&self) -> bool {
    if self.hgt.is_none() {
      return false;
    }

    return true;
  }
}
