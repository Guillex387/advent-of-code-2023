use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SubRange {
  begin: i64,
  end: i64,
  offset: i64
}

#[derive(Clone, Debug)]
struct Map {
  id: usize,
  ranges: LinkedList<SubRange>
}

fn in_range(range: &SubRange, sub_range: &SubRange) -> bool {
  let intersection = intersect_range(range, sub_range);

  intersection.begin < intersection.end
}

fn apply_offset(range: &SubRange) -> SubRange {
  SubRange {
    begin: range.begin + range.offset,
    end: range.end + range.offset,
    offset: 0
  }
}

fn intersect_range(range: &SubRange, sub_range: &SubRange) -> SubRange {
  SubRange {
    begin: range.begin.max(sub_range.begin),
    end: range.end.min(sub_range.end),
    offset: sub_range.offset
  }
}

fn exclusion_range(range: &SubRange, sub_range: &SubRange) -> LinkedList<SubRange> {
  let mut exclusion: LinkedList<SubRange> = LinkedList::new();

  if range.begin < sub_range.begin{
    exclusion.push_back(SubRange { begin: range.begin, end: sub_range.begin, offset: 0 });
  }

  if range.end > sub_range.end {
    exclusion.push_back(SubRange { begin: sub_range.end, end: range.end, offset: 0 });
  }

  exclusion
}

fn get_destination(range: &SubRange, map: &Map) -> HashSet<SubRange> {
  let mut result_set: HashSet<SubRange> = HashSet::new();

  for sub_range in map.ranges.iter() {
    if !in_range(range, sub_range) {
      continue;
    }

    result_set.insert(intersect_range(range, sub_range));

    let exclusions = exclusion_range(range, sub_range);
    for exclusion in exclusions {
      let result_part = get_destination(&exclusion, map);
      result_set.extend(result_part);
    }
  }

  if result_set.is_empty() {
    result_set.insert(range.clone());
  }

  result_set
}

fn parse_destination(ranges: HashSet<SubRange>) -> Vec<SubRange> {
  ranges.iter().map(|e| { apply_offset(e) }).collect()
}

// Input parsing

fn parse_range(line: &String) -> SubRange {
  let mut split = line.split(' ');
  let destination: i64 = split.next().unwrap().parse().unwrap();
  let origin: i64 = split.next().unwrap().parse().unwrap();
  let length: i64 = split.next().unwrap().parse().unwrap();
  
  SubRange {
    begin: origin,
    end: origin + length,
    offset: destination - origin
  }
}

fn parse_input_ranges(line: &String) -> Vec<SubRange> {
  let mut input: Vec<SubRange> = Vec::new();
  let mut first = true;
  let mut length = false;
  let mut offset = 0i64;

  for token in line.split(' ') {
    if first {
      first = false;
      continue;
    }

    if !length {
      offset = token.parse().unwrap();
      length = true;
    } else {
      length = false;
      let num: i64 = token.parse().unwrap();
      input.push(SubRange { begin: offset, end: offset + num, offset: 0 });
    }
  }

  input
}

fn parse_input(input: &String) -> (Vec<SubRange>, Vec<Map>) {
  let mut seeds: Vec<SubRange> = Vec::new();
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
      seeds = parse_input_ranges(&line.to_string());
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

fn get_min_location(seeds: &Vec<SubRange>, maps: &Vec<Map>) -> i64 {
  let mut result: Vec<SubRange> = seeds.clone();
  for map in maps {
    let mut destinations: Vec<SubRange> = Vec::new();
    for origin in result {
      destinations.extend(parse_destination(get_destination(&origin, map)));
    }
    result = destinations;
  }

  result.into_iter().reduce(|acc, e| {
    if acc.begin <= e.begin {
      acc
    } else {
      e
    }
  }).unwrap().begin
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
  println!("\n\nMin location {}", get_min_location(&seeds, &maps));
}
