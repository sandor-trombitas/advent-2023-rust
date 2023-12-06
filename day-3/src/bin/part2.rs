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

struct Gear {
    x: usize,
    y: usize,
    ratio: usize,
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

fn find_stars(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut stars = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == '*' {
                stars.push((x, y));
            }
        }
    }

    return stars;
}

fn is_symbol (c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

fn are_neighbours (a: &(usize, usize), b: &(usize, usize)) -> bool {
    let dx = (a.0 as i32 - b.0 as i32).abs();
    let dy = (a.1 as i32 - b.1 as i32).abs();

    return dx <= 1 && dy <= 1;
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
        if neighbours.len() > 0 {

            return acc + num.value;
        }

        return acc;
    });

    println!("{:?}", sum_parts);

    let stars = find_stars(&map);
    let gears: Vec<Gear> = stars.iter().filter_map(|(x, y)| {
        let mut num_neighbours = 0;
        let mut ratio = 1;
        numbers.iter().for_each(|num| {
            let mut is_neighbour = false;
            num.indices.iter().for_each(|(nx, ny)| {
                if are_neighbours(&(*x, *y), &(*nx, *ny)) {
                    is_neighbour = true;
                }
            });
            if is_neighbour {
                num_neighbours += 1;
                ratio *= num.value;
            }
        });
        if num_neighbours == 2 {
            return Some(Gear {
                x: *x,
                y: *y,
                ratio: ratio,
            });
        } else {
            return None;
        }
    }).collect();

    gears.iter().for_each(|gear| {
        println!("x: {}, y: {}, ratio: {}", gear.x, gear.y, gear.ratio);
    });

    let sum_gears = gears.iter().fold(0, |acc, gear| {
        return acc + gear.ratio;
    });

    println!("{:?}", sum_gears);
}
