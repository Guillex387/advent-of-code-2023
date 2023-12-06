use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Record {
  time: u64,
  distance: u64
}

fn calculate_distance(time: u64, boot_time: u64) -> u64 {
  let travel_time = time - boot_time;
  let speed = boot_time;

  travel_time * speed
}

fn n_of_wins(record: &Record) -> u64 {
  let mut n_wins = 0u64;

  for boot_time in 1..record.time {
    let distance = calculate_distance(record.time, boot_time);
    if distance > record.distance {
      n_wins += 1;
    }
  }

  n_wins
}

fn parse_input(input: &str) -> Record {
  let mut lines = input.lines();
  
  let time_line = lines.next().unwrap();
  let mut time = String::new();
  for time_part in time_line.split(' ') {
    if time_part.contains(':') || time_part.is_empty() {
      continue;
    }

    time += time_part;
  }

  let distance_line = lines.next().unwrap();
  let mut distance = String::new();
  for distance_part in distance_line.split(' ') {
    if distance_part.contains(':') || distance_part.is_empty() {
      continue;
    }

    distance += distance_part;
  }

  Record {
    time: time.parse().unwrap(),
    distance: distance.parse().unwrap()
  }
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

  let record = parse_input(input.as_str());
  let wins = n_of_wins(&record);

  println!("{:?}\nWins: {}", record, wins);
}
