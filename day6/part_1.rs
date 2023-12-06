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

fn parse_input(input: &str) -> Vec<Record> {
  let mut lines = input.lines();
  
  let time_line = lines.next().unwrap();
  let mut time_list: Vec<u64> = Vec::new();
  for time in time_line.split(' ') {
    if time.contains(':') || time.is_empty() {
      continue;
    }

    time_list.push(time.parse().unwrap());
  }

  let distance_line = lines.next().unwrap();
  let mut distance_list: Vec<u64> = Vec::new();
  for distance in distance_line.split(' ') {
    if distance.contains(':') || distance.is_empty() {
      continue;
    }

    distance_list.push(distance.parse().unwrap());
  }

  let mut records: Vec<Record> = Vec::new();
  records.resize(time_list.len(), Record { time: 0, distance: 0 });
  for i in 0..time_list.len() {
    records[i] = Record {
      time: time_list[i],
      distance: distance_list[i]
    };
  }

  records
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
  let records = parse_input(input.as_str());
  let mut error = 1u64;
  for record in records {
    let wins = n_of_wins(&record);
    println!("{:?}\nWins: {}", record, wins);
    error *= wins;
  }

  println!("\nError factor: {}", error);
}
