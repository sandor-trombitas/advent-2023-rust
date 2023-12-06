
use regex::Regex;

fn string_to_number (input: &String) -> i32 {
    match input.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            input.parse::<i32>().unwrap_or(0)
        }
    }
}

fn clean_input (input: &str) -> i32 {
    let re: Regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let default_value = String::from("0");
    let numbers: Vec<String> = re.find_iter(input).map(|x| String::from(x.as_str())).collect();
    let first_num = numbers.first().unwrap_or(&default_value);
    let last_num = numbers.last().unwrap_or(&default_value);
    println!("{}{}", first_num, last_num);
    let first_digit = string_to_number(first_num);
    let last_digit = string_to_number(last_num);
    println!("{}{}", first_digit, last_digit);
    let result = first_digit * 10 + last_digit;

    return result;
}
fn main() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    let input_lines:Vec<&str> = input.split('\n').collect();
    let input_values:Vec<i32> = input_lines.into_iter().map(|line| clean_input(line)).collect();
    let result = input_values.into_iter().reduce(|a, b| a + b);

    println!("{:?}", result.unwrap());
}
