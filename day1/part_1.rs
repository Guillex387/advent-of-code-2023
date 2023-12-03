use std::env;
use std::fs;
use std::u32;

// Converts a letter into a digit
fn parse_digit(letter: char) -> Option<u32> {
  if letter < '0' || letter > '9' {
    None
  } else {
    Some(letter as u32 - '0' as u32)
  }
}

// Gets the calibration of a line
fn get_calibration(line: &String) -> u32 {
  let (mut first, mut last) = (0u32, 0u32);
  let mut first_find = true;

  for letter in line.chars() {
    let parse_letter = parse_digit(letter);
    if parse_letter.is_none() {
      continue;
    }

    let digit = parse_letter.unwrap();

    if first_find {
      first = digit;
      first_find = false;
    }

    last = digit;
  }

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
