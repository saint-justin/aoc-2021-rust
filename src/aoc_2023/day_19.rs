use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn accepted_part_rating_sum(input: &Vec<&str>) -> u32 {
  let re_parts = Regex::new(r"([0-9]{1,})").unwrap();
  let re_workflows = Regex::new(r"([a-zA-Z0-9<>:]{1,})").unwrap();
  let re_workflow_parts = Regex::new(r"([a-zA-Z0-9]{1,})").unwrap();

  let mut parts: Vec<Part> = Vec::new();
  let mut workflows: HashMap<String, Vec<String>> = HashMap::new();

  for line in input {
    if line.len() == 0 {
      continue;
    } else if line.chars().nth(0).unwrap() == '{' {
      let (x, m, a, s) = re_parts
        .find_iter(line)
        .map(|s| s.as_str().parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();
      parts.push(Part { x, m, a, s });
    } else {
      let mut sliced_workflow = re_workflows
        .find_iter(line)
        .map(|m| m.as_str().to_owned())
        .collect_vec();
      let key = sliced_workflow.remove(0);
      workflows.insert(key.to_owned(), sliced_workflow);
    }
  }

  let mut accepted: Vec<Part> = Vec::new();
  for part in parts {
    let mut current = "in".to_owned();
    loop {
      match current.as_str() {
        "R" => break,
        "A" => {
          accepted.push(part);
          break;
        }
        _ => {
          current = evaluate_workflow(&part, workflows.get(&current).unwrap(), &re_workflow_parts)
        }
      }
    }
  }

  accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

fn evaluate_workflow(part: &Part, workflows: &Vec<String>, re: &Regex) -> String {
  for flow in workflows {
    match flow.as_str() {
      "A" => return "A".to_owned(),
      "R" => return "R".to_owned(),
      _ => {
        let matches = re.find_iter(&flow).map(|m| m.as_str()).collect_vec();
        if matches.len() == 1 {
          return matches[0].to_owned();
        }

        let (part_type, comparator, next_tag) = matches.iter().collect_tuple().unwrap();
        let part_value = match *part_type {
          "x" => part.x,
          "m" => part.m,
          "a" => part.a,
          "s" => part.s,
          _ => panic!("Invalid part type passed: {}", part_type),
        };
        let comparator_value = comparator.parse::<u32>().unwrap();
        if flow.contains(">") {
          if part_value > comparator_value {
            return next_tag.to_owned().to_owned();
          }
        } else {
          if part_value < comparator_value {
            return next_tag.to_owned().to_owned();
          }
        }
      }
    }
  }

  panic!(
    "Invalid workflow evaluation state for part {:?}: {:?}",
    part, workflows
  );
}

#[derive(Debug, Clone, Copy)]
struct Part {
  x: u32,
  m: u32,
  a: u32,
  s: u32,
}
