use std::collections::LinkedList;
use std::env;
use std::fs;
use std::usize;

#[derive(Clone, Debug)]
struct Range {
  origin: u64,
  destination: u64,
  length: u64
}

#[derive(Clone, Debug)]
struct Map {
  id: usize,
  ranges: LinkedList<Range>
}

fn in_range(origin: u64, range: &Range) -> bool {
  origin >= range.origin && origin < (range.origin + range.length)
}

fn get_destination(origin: u64, map: &Map) -> u64 {
  for range in map.ranges.iter() {
    if !in_range(origin, range) {
      continue;
    }
    
    let offset = origin - range.origin;
    return range.destination + offset;
  }
  
  origin
}

fn parse_range(line: &String) -> Range {
  let mut split = line.split(' ');
  let destination: u64 = split.next().unwrap().parse().unwrap();
  let origin: u64 = split.next().unwrap().parse().unwrap();
  let length: u64 = split.next().unwrap().parse().unwrap();
  
  Range { origin, destination, length }
}

fn parse_input(input: &String) -> (Vec<u64>, Vec<Map>) {
  let mut seeds: Vec<u64> = Vec::new();
  let mut maps: Vec<Map> = Vec::new();

  let mut map = Map {
    id: 0,
    ranges: LinkedList::new()
  };
  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    if seeds.is_empty() {
      let mut first = true;
      for token in line.split(' ') {
        if token.is_empty() {
          continue;
        }
        
        if first {
          first = false;
          continue;
        }

        let seed: u64 = token.parse().unwrap();
        seeds.push(seed);
      }
      continue;
    }

    if !line.contains("map:") {
      map.ranges.push_back(parse_range(&line.to_string()));
    } else if map.id != 0 {
      maps.push(map.clone());
      map.ranges.clear();
      map.id += 1;
    } else {
      map.id += 1;
    }

  }

  maps.push(map);

  (seeds, maps)
}

fn print_maps(maps: &Vec<Map>) {
  for map in maps {
    println!("Id {}:", map.id);
    for range in map.ranges.iter() {
      println!("{:?}", range);
    }
  }
}

fn get_min_location(seeds: &Vec<u64>, maps: &Vec<Map>) -> u64 {
  let mut result: Vec<u64> = seeds.clone();
  for map in maps {
    let mut destinations: Vec<u64> = Vec::new();
    for origin in result {
      destinations.push(get_destination(origin, map));
    }
    result = destinations;
  }

  result.iter().min().unwrap().clone()
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

  let (seeds, maps) = parse_input(&input);
  println!("{:?}", seeds);
  print_maps(&maps);
  println!("Min location {}", get_min_location(&seeds, &maps));
}
