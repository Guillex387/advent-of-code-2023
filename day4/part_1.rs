use std::collections::HashSet;
use std::env;
use std::fs;
use std::{u32, u64};

#[derive(Debug)]
struct Card {
  id: usize,
  winners: HashSet<u32>,
  numbers: HashSet<u32>
}

// Parse the card input into a structure data
fn parse_card(card: &String) -> Card {
  let mut card_token = false;
  let mut reading_winners = true;
  let mut result = Card {
    id: 0,
    winners: HashSet::new(),
    numbers: HashSet::new()
  };

  for token in card.split([' ', ':']) {
    if token.is_empty() {
      continue;
    }

    if token == "Card" {
      card_token = true;
      continue;
    }

    if card_token {
      card_token = false;
      result.id = token.parse().unwrap();
      continue;
    }

    if token == "|" {
      reading_winners = false;
      continue;
    }

    let number: u32 = token.parse().unwrap();
    if reading_winners {
      result.winners.insert(number);
    } else {
      result.numbers.insert(number);
    }
  }

  result
}

// Calculates the points of a card
fn calculate_card_points(card: &Card) -> u64 {
  let number_winners = card.numbers.intersection(&card.winners).count();
  
  if number_winners == 0 {
    0
  } else {
    2u64.pow(number_winners as u32 - 1)
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

  let mut total_points = 0u64;

  for line in input.lines() {
    let card = parse_card(&line.to_string());
    let points = calculate_card_points(&card);
    total_points += points;
    println!("{:?}\npoints: {}", card, points);
  }

  println!("Total points: {}", total_points); 
}
