use std::fs;
use std::io::{BufReader, BufRead};

fn read_file_to_2d(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    return reader.lines()
      .map(|line| line.expect("Could not read line")).collect::<Vec<String>>();
}
#[derive(Debug)]
struct Card {
  id: u32,
  winning: Vec<u32>,
  drawn: Vec<u32>,
  points: u32
}

fn calc_points(winning: &Vec<u32>, drawn: &Vec<u32>) -> u32 {
  let mut points = 0;
  for (i, num) in winning.iter().enumerate() {
    if drawn.contains(num) {
      if points == 0 {
        points = 1;
      } else {
        points *= 2;
      }
    }
  }

  return points;
}

fn line_to_card(line: &String) -> Card {
  let mut scratch = line.split(": ");
  let id = scratch.next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
  let mut numbers = scratch.next().unwrap().split(" | ");
  // println!("My numbers: {:?} \n", numbers);
  let winning: Vec<u32> = numbers.next().unwrap()
    .trim().split(" ")
    .filter_map(|x| {
      let res = x.parse::<u32>();

      match res {
        Ok(num) => Some(num),
        Err(_) => None
      }
    }).collect();
  let drawn: Vec<u32> = numbers.next().unwrap()
    .trim().split(" ")
    .filter_map(|x| {
      let res = x.parse::<u32>();

      match res {
        Ok(num) => Some(num),
        Err(_) => None
      }
    }).collect();

  Card {
    id: id,
    winning: winning.clone(),
    drawn: drawn.clone(),
    points: calc_points(&winning, &drawn)
  }
}

fn main() {
    let input = read_file_to_2d("input.txt");
    let cards = input.iter().map(|line| line_to_card(line)).collect::<Vec<Card>>();
    let points = cards.iter().fold(0, |acc, card| acc + card.points);

    cards.iter().for_each(|card| println!("{:?}", card));
    println!("Total points: {}", points);
}