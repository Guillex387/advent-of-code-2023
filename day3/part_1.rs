use std::collections::LinkedList;
use std::env;
use std::fs;
use std::u64;
use std::usize;

#[derive(Clone)]
struct EngineNumber {
  row: usize,
  begin: usize,
  end: usize,
  number: u64
}

struct EngineSymbol {
  row: usize,
  pos: usize,
  symbol: char
}

fn is_digit(letter: char) -> bool {
  letter >= '0' && letter <= '9'
}

// Parse the input to two list, one of engine numbers and other of engine symbols
fn parse_engine(input: &String) -> (LinkedList<EngineNumber>, LinkedList<EngineSymbol>) {
  let mut numbers: LinkedList<EngineNumber> = LinkedList::new();
  let mut symbols: LinkedList<EngineSymbol> = LinkedList::new();

  for (row, line) in input.lines().enumerate() {
    let mut number_str = String::new();
    for (i, cell) in line.chars().enumerate() {
      if is_digit(cell) {
        number_str.push(cell);
        continue;
      }

      if !number_str.is_empty() {
        let number: u64 = number_str.parse().unwrap();
        let begin = i - number_str.len();
        let end = i - 1;
        number_str.clear();
        numbers.push_back(EngineNumber { row, begin, end, number });
      }
     
      if cell == '.' {
        continue;
      }

      symbols.push_back(EngineSymbol { row, pos: i, symbol: cell });
    }

    if !number_str.is_empty() {
      let number: u64 = number_str.parse().unwrap();
      let begin = line.len() - number_str.len();
      let end = line.len() - 1;
      number_str.clear();
      numbers.push_back(EngineNumber { row, begin, end, number });
    }
  }

  (numbers, symbols)
}

// Checks if a number is adyacent to a symbol
fn is_adyacent(number: &EngineNumber, symbol: &EngineSymbol) -> bool {
  let begin = if number.begin == 0 {
      number.begin
    } else {
      number.begin - 1
    };

  if number.row == symbol.row {
    (number.end + 1) == symbol.pos || begin == symbol.pos
  } else if number.row.abs_diff(symbol.row) == 1 {
    symbol.pos <= (number.end + 1) && symbol.pos >= begin
  } else {
    false
  }
}

// Filters the numbers that are adyacent to one or more symbols in the list
fn calculate_adyacent(numbers: &LinkedList<EngineNumber>, symbols: &LinkedList<EngineSymbol>) -> LinkedList<EngineNumber> {
  let mut result: LinkedList<EngineNumber> = LinkedList::new();
  for number in numbers {
    for symbol in symbols {
      if is_adyacent(number, symbol) {
        println!("{} adyacent to {}", number.number, symbol.symbol);
        result.push_back(number.clone());
        break;
      }
    }
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

  let (numbers, symbols) = parse_engine(&input);
  let adyacent_numbers = calculate_adyacent(&numbers, &symbols);
  let mut adyacent_sum = 0u64;
  for number in adyacent_numbers {
    adyacent_sum += number.number;
  }
  println!("Adyacent sum: {}", adyacent_sum);
}
