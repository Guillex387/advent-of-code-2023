use std::env;
use std::fs;
use std::usize;

struct Roll {
  red: usize,
  green: usize,
  blue: usize
}

struct Game {
  id: usize,
  rolls: Vec<Roll>
}

const MAX_AMOUNTS: Roll = Roll {
  red: 12,
  green: 13,
  blue: 14
};

// Parse a roll string into a roll struct
fn parse_roll(roll: &String) -> Roll {
  let mut result = Roll {
    red: 0,
    green: 0,
    blue: 0
  };
  let mut amount_token = true;
  let mut amount = 0usize;

  for token in roll.replace(';', "").split([' ', ',']) {
    if token.is_empty() {
      continue;
    }

    if amount_token {
      amount_token = false;
      amount = token.parse().unwrap();
    } else {
      amount_token = true;
      match token {
        "red" => result.red += amount,
        "green" => result.green += amount,
        "blue" => result.blue += amount,
        _ => ()
      }
    }
  }

  result
}

// Parse text line to a game struct
fn parse_game(line: &String) -> Game {
  let mut game_token = false;
  let mut id = 0usize;
  let mut roll_token = String::new();
  let mut rolls: Vec<Roll> = Vec::new();
  for token in line.split([' ', ':']) {
    if token == "Game" {
      game_token = true;
      continue;
    }

    if game_token {
      game_token = false;
      id = token.parse().unwrap();
      continue;
    }

    roll_token += token;
    roll_token += " ";

    if token.contains(';') {
      rolls.push(parse_roll(&roll_token));
      roll_token.clear();
    }
  }

  if !roll_token.is_empty() {
    rolls.push(parse_roll(&roll_token));
  }

  Game { id, rolls }
}

// Return if the game is possible with certain amount of cubes
fn is_possible(game: &Game, max_amounts: &Roll) -> bool {
  for roll in game.rolls.iter() {
    if roll.red > max_amounts.red {
      return false;
    } else if roll.green > max_amounts.green {
      return false;
    } else if roll.blue > max_amounts.blue {
      return false;
    }
  }

  true
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

  let mut id_sum = 0usize;
  for line in input.lines() {
    let game = parse_game(&line.to_string());
    let possible = is_possible(&game, &MAX_AMOUNTS);

    println!("Game {} {}", game.id, possible);

    if !possible {
      continue;
    }
    id_sum += game.id;
  }

  println!("Sum of possible games {}", id_sum);
}
