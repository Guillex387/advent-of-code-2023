use std::env;
use std::fs;

fn predict_next(lectures: &Vec<i64>) -> i64 {
  let mut differences: Vec<i64> = Vec::new();
  differences.reserve(lectures.len() - 1);
  for i in 0..lectures.len() - 1 {
    differences.push(lectures[i + 1] - lectures[i]);
  }
  
  println!("{:?} diff", differences);

  if differences.iter().all(|num| { *num == 0 }) {
    lectures.first().unwrap().clone()
  } else {
    let predict_diff = predict_next(&differences);
    lectures.last().unwrap() + predict_diff
  }
}

fn parse_line(line: &str) -> Vec<i64> {
  line.split(' ')
    .filter(|e| { !e.is_empty() })
    .map(|token| { token.parse().unwrap() })
    .collect()
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

  let mut total_prediction = 0i64;
  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    let lectures = parse_line(line);
    println!("{:?} lectures", lectures);
    let prediction = predict_next(&lectures);
    println!("{} prediction\n", prediction);
    total_prediction += prediction;
  }

  println!("Total prediction {}", total_prediction);
}
