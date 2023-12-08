use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;

const CARD_ORD: &str = "23456789TJQKA";

#[derive(Debug, Clone)]
struct Hand {
  cards: String,
  bid: u64,
  hand_type: u64
}

// Return a number between 0 and 6, Where 6 is the max range of a hand
fn get_type(cards: &String) -> u64 {
  let mut map: HashMap<char, u64> = HashMap::new();

  for letter in cards.chars() {
    let repeat = map.get_mut(&letter);
    if repeat.is_none() {
      map.insert(letter, 1);
      continue;
    }
    let repeat_value = repeat.unwrap();
    *repeat_value += 1;
  }

  let mut pair_1 = false;
  let mut pair_2 = false;
  let mut three_pair = false;
  for (_, repeat) in map {
    if repeat > 3 {
      return repeat + 1;
    }
    if repeat == 2 {
      if pair_1 {
        pair_2 = true;
      } else {
        pair_1 = true;
      }
    } else if repeat == 3 {
      three_pair = true;
    }
  }

  if three_pair {
    if pair_1 {
      4
    } else {
      3
    }
  } else if pair_1 {
    if pair_2 {
      2
    } else {
      1
    }
  } else {
    0
  }
}

fn cmp_hand(hand_a: &Hand, hand_b: &Hand) -> Ordering {
  if hand_a.hand_type > hand_b.hand_type {
    return Ordering::Greater;
  } else if hand_a.hand_type < hand_b.hand_type {
    return Ordering::Less;
  }

  let mut cards_b = hand_b.cards.chars();
  for card_a in hand_a.cards.chars() {
    let card_b = cards_b.next().unwrap();
    let card_a_index = CARD_ORD.find(card_a).unwrap();
    let card_b_index = CARD_ORD.find(card_b).unwrap();

    let comparisson = card_a_index.cmp(&card_b_index);
    if comparisson == Ordering::Equal {
      continue;
    }
    return comparisson;
  }

  Ordering::Equal
}

fn get_total_winnings(hands: &mut Vec<Hand>) -> u64 {
  let mut total_winnings = 0u64;
  hands.sort_by(cmp_hand);

  for (i, hand) in hands.iter().enumerate() {
    let rank = (i + 1) as u64;
    let wins = rank * hand.bid;
    println!("{:?} rank {} wins {}", hand, rank, wins);
    total_winnings += wins;
  }

  total_winnings
}

fn parse_input(input: &str) -> Vec<Hand> {
  let mut result: Vec<Hand> = Vec::new();

  for line in input.lines() {
    let mut tokens = line.split(' ');
    let cards: String = tokens.next().unwrap().to_string();
    let bid: u64 = tokens.next().unwrap().parse().unwrap();
    let hand_type = get_type(&cards);
    result.push(Hand { cards, bid, hand_type });
  }

  result
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
  let mut hands = parse_input(input.as_str());
  println!("Winnings {}", get_total_winnings(&mut hands));
}
