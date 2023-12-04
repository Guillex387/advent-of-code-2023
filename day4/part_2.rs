use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::usize;
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

// Calculates the number of winners
fn card_winners(card: &Card) -> usize {
  card.numbers.intersection(&card.winners).count()
}

// Calculates the total cards winned
fn calculate_winned_cards(cards: &LinkedList<Card>) -> u64 {
  let mut copies: Vec<u64> = Vec::new();
  copies.resize(cards.len(), 1);

  for card in cards {
    let card_wins = card_winners(card);
    for id in card.id..card.id+card_wins {
      copies[id] += copies[card.id - 1];
    }
  }

  copies.iter().sum()
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

  let mut card_list: LinkedList<Card> = LinkedList::new();

  for line in input.lines() {
    let card = parse_card(&line.to_string());
    card_list.push_back(card);
  }

  let total_cards = calculate_winned_cards(&card_list);
  println!("Total cards: {}", total_cards);
}
