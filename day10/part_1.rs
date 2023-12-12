use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Tile {
  VPipe,
  HPipe,
  NorthEastBend,
  NorthWestBend,
  SouthWestBend,
  SouthEastBend,
  Ground,
  Start
}

enum Dir {
  North,
  South,
  East,
  West
}

#[derive(Debug, Clone)]
struct Compatible {
  top: bool,
  bottom: bool,
  left: bool,
  right: bool
}

#[derive(Debug, Clone)]
struct Terrain {
  start: (usize, usize),
  matrix: Vec<Vec<Tile>>
}

fn get_tile(pos: &(usize, usize), terrain: &Terrain) -> Tile {
  if pos.0 >= terrain.matrix.len() || pos.1 >= terrain.matrix.first().unwrap().len() {
    return Tile::Ground;
  }
  terrain.matrix[pos.0][pos.1].clone()
}

fn advance(pos: &(usize, usize), dir: Dir) -> (usize, usize) {
  match dir {
    Dir::North => (pos.0.overflowing_sub(1).0, pos.1),
    Dir::South => (pos.0.overflowing_add(1).0, pos.1),
    Dir::East => (pos.0, pos.1.overflowing_add(1).0),
    Dir::West => (pos.0, pos.1.overflowing_sub(1).0)
  }
}

fn get_compatibility(tile: &Tile) -> Compatible {
  match tile {
    Tile::VPipe => Compatible {
      top: true,
      bottom: true,
      left: false,
      right: false
    },
    Tile::HPipe => Compatible {
      top: false,
      bottom: false,
      left: true,
      right: true
    },
    Tile::NorthEastBend => Compatible {
      top: true,
      bottom: false,
      left: false,
      right: true
    },
    Tile::NorthWestBend => Compatible {
      top: true,
      bottom: false,
      left: true,
      right: false
    },
    Tile::SouthWestBend => Compatible {
      top: false,
      bottom: true,
      left: true,
      right: false
    },
    Tile::SouthEastBend => Compatible {
      top: false,
      bottom: true,
      left: false,
      right: true
    },
    Tile::Start => Compatible {
      top: true,
      bottom: true,
      left: true,
      right: true
    },
    _ => Compatible {
      top: false,
      bottom: false,
      left: false,
      right: false
    }
  }
}

fn possible_dir(pos: &(usize, usize), terrain: &Terrain) -> Vec<(usize, usize)> {
  let mut possible_dir: Vec<(usize, usize)> = Vec::new();

  let tile = get_tile(pos, terrain);
  let main_comp = get_compatibility(&tile);

  let north_pos = advance(pos, Dir::North);
  let north_tile = get_tile(&north_pos, terrain);
  let north_comp = get_compatibility(&north_tile);
  if main_comp.top && north_comp.bottom {
    possible_dir.push(north_pos);
  }

  let south_pos = advance(pos, Dir::South);
  let south_tile = get_tile(&south_pos, terrain);
  let south_comp = get_compatibility(&south_tile);
  if main_comp.bottom && south_comp.top {
    possible_dir.push(south_pos);
  }

  let east_pos = advance(pos, Dir::East);
  let east_tile = get_tile(&east_pos, terrain);
  let east_comp = get_compatibility(&east_tile);
  if main_comp.right && east_comp.left {
    possible_dir.push(east_pos);
  }

  let west_pos = advance(pos, Dir::West);
  let west_tile = get_tile(&west_pos, terrain);
  let west_comp = get_compatibility(&west_tile);
  if main_comp.left && west_comp.right {
    possible_dir.push(west_pos);
  }

  possible_dir
}

fn path_finder(pos: &(usize, usize), terrain: &Terrain, path: &mut HashSet<(usize, usize)>) {
  if !path.insert(pos.clone()) {
    return;
  }

  let adyacents = possible_dir(pos, terrain);
  
  for adyacent in adyacents {
    path_finder(&adyacent, terrain, path);
  }
}

fn clean_terrain(terrain: &mut Terrain, path: HashSet<(usize, usize)>) {
  for (i, row) in terrain.matrix.iter_mut().enumerate() {
    for (j, tile) in row.iter_mut().enumerate() {
      if !path.contains(&(i, j)) {
        *tile = Tile::Ground;
      }
    }
  }
}

fn max_distance_calculator(terrain: &Terrain) -> u64 {
  let mut positions: Vec<(usize, usize)> = Vec::from([terrain.start.clone()]);
  let mut visited: HashSet<(usize, usize)> = HashSet::from([terrain.start.clone()]);
  let mut distance = 0u64;

  loop {
    let mut new_positions: Vec<(usize, usize)> = Vec::new();
    for pos in positions.iter() {
      for new_pos in possible_dir(pos, terrain) {
        if !visited.insert(new_pos) {
          continue;
        }
        new_positions.push(new_pos);
      }
    }
    println!("Positions {:?} {}", positions, distance);
    if new_positions.is_empty() {
      break;
    }
    positions = new_positions;
    distance += 1;
  }

  distance
}

// Input parsing

fn parse_tile(tile: char) -> Tile {
  match tile {
    '|' => Tile::VPipe,
    '-' => Tile::HPipe,
    'L' => Tile::NorthEastBend,
    'J' => Tile::NorthWestBend,
    '7' => Tile::SouthWestBend,
    'F' => Tile::SouthEastBend,
    'S' => Tile::Start,
    _ => Tile::Ground
  }
}

fn parse_line(line: &str) -> Vec<Tile> {
  line
    .chars()
    .map(|letter| { parse_tile(letter) })
    .collect()
}

fn find_start(matrix: &Vec<Vec<Tile>>) -> (usize, usize) {
  for (i, row) in matrix.iter().enumerate() {
    for (j, tile) in row.iter().enumerate() {
      match tile {
        Tile::Start => return (i, j),
        _ => ()
      }
    }
  }

  (0, 0)
}

fn parse_input(input: &str) -> Terrain {
  let matrix: Vec<Vec<Tile>> = input
    .lines()
    .map(|line| { parse_line(line) })
    .collect();

  Terrain {
    start: find_start(&matrix),
    matrix
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

  let mut terrain = parse_input(input.as_str());
  let mut path: HashSet<(usize, usize)> = HashSet::new();
  path_finder(&terrain.start, &terrain, &mut path);
  clean_terrain(&mut terrain, path);

  max_distance_calculator(&terrain);
}
