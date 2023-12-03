use std::env;
use std::fs;
use std::string::String;
use std::u32;
use std::usize;

// A const vector with all the patterns
const NUMBERS_STR: [&str ; 18] = [
    "one",
    "two",
    "three", 
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9"
  ];

fn reverse_string(string: &String) -> String {
  string.chars().rev().collect()
}

// Parse the pattern index to a digit
fn parse_pattern(pattern: usize) -> u32 {
  (pattern as u32 % 9u32) + 1u32  
}

// Searchs the first pattern in the line
fn find_first(line: &String) -> Option<usize> {
  let mut last_match_pos = usize::MAX;
  let mut last_match_pattern = usize::MAX;

  for (i, pattern) in NUMBERS_STR.iter().enumerate() {
    let find_result = line.find(pattern);
    if find_result.is_none() {
      continue;
    }

    let pos = find_result.unwrap();
    if pos > last_match_pos {
      continue;
    }

    last_match_pos = pos;
    last_match_pattern = i;
  }

  if last_match_pattern == usize::MAX {
    None
  } else {
    Some(last_match_pattern)
  }
}

// Searchs the last pattern in the line
fn find_last(line: &String) -> Option<usize> {
  let mut last_match_pos = usize::MAX;
  let mut last_match_pattern = usize::MAX;
  let rev_line = reverse_string(line);

  for (i, pattern) in NUMBERS_STR.iter().enumerate() {
    let rev_pattern = reverse_string(&pattern.to_string());
    let find_result = rev_line.find(rev_pattern.as_str());
    if find_result.is_none() {
      continue;
    }

    let pos = find_result.unwrap();
    if pos > last_match_pos {
      continue;
    }

    last_match_pos = pos;
    last_match_pattern = i;
  }

  if last_match_pattern == usize::MAX {
    None
  } else {
    Some(last_match_pattern)
  }
}

// Gets the calibration of the line
fn get_calibration(line: &String) -> u32 {
  let first_index = find_first(line).unwrap();
  let last_index = find_last(line).unwrap();

  let (first, last) = (parse_pattern(first_index), parse_pattern(last_index));

  first * 10 + last
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Not enough arguments");
    return;
  }

  let file_name = &args[1];
  let buffer: String = fs::read_to_string(file_name)
    .expect("Error reading the input file");

  let mut global_calibration = 0u32;

  for line_str in buffer.lines() {
    let line = line_str.to_string();
    let local_calibration = get_calibration(&line);
    global_calibration += local_calibration;
    println!("'{}' {}", line, local_calibration);
  }

  println!("Calibration is {}", global_calibration);
}
