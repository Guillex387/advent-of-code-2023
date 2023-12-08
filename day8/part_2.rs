use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Instruction {
  LEFT,
  RIGHT
}

#[derive(Debug)]
struct InstructionSet {
  next: usize,
  sequence: Vec<Instruction>
}

#[derive(Debug, Clone)]
struct Path {
  origin: String,
  destination: (String, String)
}

#[derive(Debug)]
struct PathMap {
  begin: Vec<String>,
  map: HashMap<String, (String, String)>
}

fn is_begin_path(path: &String) -> bool {
  path.ends_with('A')
}

fn is_end_path(path: &String) -> bool {
  path.ends_with('Z')
}

fn lcm(a: u64, b: u64) -> u64 {
  let mut result = 0u64;
  let mut a_copy = a;
  let mut b_copy = b;

  while a_copy % b_copy > 0 {
    result = a_copy % b_copy; 
    a_copy = b_copy;
    b_copy = result;
  }

  (a * b) / result
}

fn get_instruction(instruction_set: &mut InstructionSet) -> Instruction {
  let actual_instruction = instruction_set.next;
  instruction_set.next = (actual_instruction + 1) % instruction_set.sequence.len();
  instruction_set.sequence[actual_instruction].clone()
}

fn count_steps(instruction_set: &mut InstructionSet, path_map: &PathMap) -> u64 {
  // Make the least common multiple
  // to calculate in which step the n locations
  // are all end locations
  path_map.begin.iter().map(|location| {
    let mut actual_location = location.clone();
    let mut steps = 0u64;
    while !is_end_path(&actual_location) {
      let instruction = get_instruction(instruction_set);
      let possible_locations = path_map.map[&actual_location].clone();
      actual_location = match instruction {
        Instruction::LEFT => possible_locations.0,
        Instruction::RIGHT => possible_locations.1
      };
      steps += 1;
    }
    println!("{} -> {} steps {}", location, actual_location, steps);
    steps
  }).reduce(|acc, steps| { lcm(acc, steps) }).unwrap()
}

// Input parsing

fn parse_instructions(line: &str) -> InstructionSet {
  let mut instruction_set = InstructionSet {
    next: 0,
    sequence: Vec::new()
  };

  for letter in line.chars() {
    let instruction: Instruction = match letter {
      'L' => Instruction::LEFT,
      'R' => Instruction::RIGHT,
      _ => continue
    };
    instruction_set.sequence.push(instruction);
  }

  instruction_set
}

fn parse_path(line: &str) -> Path {
  let mut iter = line.split([' ', '=', '(', ')', ',']).filter(|token| {
    !token.is_empty()
  });

  Path {
    origin: iter.next().unwrap().to_string(),
    destination: (
      iter.next().unwrap().to_string(),
      iter.next().unwrap().to_string()
    )
  }
}

fn parse_input(input: &str) -> (InstructionSet, PathMap) {
  let mut first = true;
  let mut instruction_set = InstructionSet {
    next: 0,
    sequence: Vec::new()
  };
  let mut path_map = PathMap {
    map: HashMap::new(),
    begin: Vec::new(),
  };

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    if first {
      first = false;
      instruction_set = parse_instructions(line);
      continue;
    }

    let path = parse_path(line);

    if is_begin_path(&path.origin) {
      path_map.begin.push(path.origin.clone());
    }

    path_map.map.insert(path.origin, path.destination);
  }

  (instruction_set, path_map)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Not enogh arguments");
    return;
  }

  let file_name = &args[1];
  let input = fs::read_to_string(file_name)
    .expect("Error reading the file");
  let (mut instruction_set, path_map) = parse_input(input.as_str());
  println!("Steps {}", count_steps(&mut instruction_set, &path_map));
}
