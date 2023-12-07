use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn open_file_and_convert_to_string(path: String) -> String {
    let file = File::open(path).expect("Cannot open file");

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader
        .read_to_string(&mut contents)
        .expect("Cannot read file");

    contents
}

fn split_string_into_lines(input: String) -> Vec<String> {
    let splitted: Vec<&str> = input.split("\n").collect();
    let mut lines = Vec::new();

    for line in splitted {
        lines.push(line.to_string());
    }

    lines
}

fn convert_spelled_out_number_to_int(input: String) -> u32 {
    let mut number = 0;

    match input.as_str() {
        "1" | "one" => number = 1,
        "2" | "two" => number = 2,
        "3" | "three" => number = 3,
        "4" | "four" => number = 4,
        "5" | "five" => number = 5,
        "6" | "six" => number = 6,
        "7" | "seven" => number = 7,
        "8" | "eight" => number = 8,
        "9" | "nine" => number = 9,
        _ => println!("No match"),
    }

    number
}

fn find_first_and_last_digit_in_string(input: String) -> (u32, u32) {
    let match_first_digit =
        Regex::new(r"^(?:.*?)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|0)")
            .unwrap();
    let match_last_digit =
        Regex::new(r"^(?:.*)(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|0)")
            .unwrap();

    let first_digit = match_first_digit
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let last_digit = match_last_digit
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let first_digit = convert_spelled_out_number_to_int(first_digit.to_string());
    let last_digit = convert_spelled_out_number_to_int(last_digit.to_string());

    (first_digit, last_digit)
}

fn combine_digits(first_digit: u32, last_digit: u32) -> u32 {
    let combined = format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .unwrap();

    combined
}

fn main() {
    let current_path = std::env::current_dir().unwrap();
    let current_path_string = current_path.to_str().unwrap();

    let path = format!("{}/src/input_part_2.txt", current_path_string);

    let input = open_file_and_convert_to_string(path);
    let lines = split_string_into_lines(input);

    let mut sum_of_digits = 0;

    for line in &lines {
        if line.is_empty() {
            continue;
        }

        let (first_digit, last_digit) = find_first_and_last_digit_in_string(line.to_string());
        let combined = combine_digits(first_digit, last_digit);

        sum_of_digits += combined;
    }

    println!("Sum of digits: {}", sum_of_digits);
}
