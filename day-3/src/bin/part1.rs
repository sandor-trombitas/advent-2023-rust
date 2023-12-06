use std::fs;
use std::io::{BufReader, BufRead};

fn read_file_to_2d(filename: &str) -> Vec<Vec<char>> {
    let file = fs::File::open(filename).expect("file not found");
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        map.push(line.chars().collect::<Vec<char>>());
    }

    return map;
}

struct Num {
    indices: Vec<(usize, usize)>,
    value: usize,
}

fn find_numbers(map: &Vec<Vec<char>>) -> Vec<Num> {
    let mut numbers = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut number = String::new();
        let mut indices: Vec<(usize, usize)> = Vec::new();
        for (x, col) in row.iter().enumerate() {
            if col.is_digit(10) {
                indices.push((x, y));
                number.push(*col);
            } else {
                if number.len() > 0 {
                    numbers.push(Num {
                        indices: indices,
                        value: number.parse::<usize>().unwrap(),
                    });
                    number = String::new();
                    indices = Vec::new();
                }
            }
        }
        if number.len() > 0 {
            numbers.push(Num {
                indices: indices,
                value: number.parse::<usize>().unwrap(),
            });
        }
    }

    return numbers;
}

fn is_symbol (c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

fn find_neighbours(map: &Vec<Vec<char>>, num: &Num) -> Vec<char> {
    let mut neighbours = Vec::new();
    let possible_neighbours = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    num.indices.iter().for_each(|(x, y)| {
        possible_neighbours.iter().for_each(|(dx, dy)| {
            let x = *x as i32 + dx;
            let y = *y as i32 + dy;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                if y < map.len() && x < map[y].len() && is_symbol(map[y][x]) {
                    neighbours.push(map[y][x]);
                }
            }
        });
    });

    return neighbours.into_iter().filter(|c| !c.is_digit(10)).collect();
}

fn main() {
    let map = read_file_to_2d("./input");
    let numbers = find_numbers(&map);

    let sum_parts = numbers.iter().fold(0, |acc, num| {
        let neighbours = find_neighbours(&map, num);
        print!("{:?} ", num.value);
        println!("{:?}", neighbours);

        if neighbours.len() > 0 {
            println!("{:?}", num.value);
            println!("{:?}", neighbours);
            return acc + num.value;
        }

        return acc;
    });

    println!("{:?}", sum_parts);
}
