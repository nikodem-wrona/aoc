use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file() -> io::Result<Vec<String>> {
    let mut path = env::current_dir()?;
    path.push("src/input_part_2.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn check_if_game_is_possible_for_hand(reds: i32, greens: i32, blues: i32) -> bool {
    let max_number_of_reds = 12;
    let max_number_of_greens = 13;
    let max_number_of_blues = 14;

    if reds > max_number_of_reds
        || greens > max_number_of_greens
        || blues > max_number_of_blues
        || reds < 0
        || greens < 0
        || blues < 0
    {
        return false;
    }

    true
}

fn get_results_for_game(game: String) -> (i32, Vec<bool>) {
    let parts: Vec<&str> = game.split(':').collect();
    let game_id = parts[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let hands: Vec<&str> = parts[1].split(';').collect();

    let mut hand_results = Vec::new();

    for hand in hands {
        let cubes: Vec<&str> = hand.split(',').collect();

        let mut number_of_reds = 0;
        let mut number_of_greens = 0;
        let mut number_of_blues = 0;

        for cube in cubes {
            let cube_parts: Vec<&str> = cube.trim().split(' ').collect();
            let color = cube_parts[1];
            let count = cube_parts[0].parse::<i32>().unwrap();

            match color {
                "red" => number_of_reds += count,
                "green" => number_of_greens += count,
                "blue" => number_of_blues += count,
                _ => (),
            }
        }

        hand_results.push(check_if_game_is_possible_for_hand(
            number_of_reds,
            number_of_greens,
            number_of_blues,
        ));
    }

    (game_id, hand_results)
}

fn get_power_of_colors(game: String) -> (i32) {
    let parts: Vec<&str> = game.split(':').collect();
    let game_id = parts[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let hands: Vec<&str> = parts[1].split(';').collect();

    let mut max_value_of_red = 0;
    let mut max_value_of_blue = 0;
    let mut max_value_of_green = 0;

    for hand in hands {
        let cubes: Vec<&str> = hand.split(',').collect();

        for cube in cubes {
            let cube_parts: Vec<&str> = cube.trim().split(' ').collect();
            let color = cube_parts[1];
            let count = cube_parts[0].parse::<i32>().unwrap();

            match color {
                "red" => {
                    if count > max_value_of_red {
                        max_value_of_red = count;
                    }
                }
                "green" => {
                    if count > max_value_of_green {
                        max_value_of_green = count;
                    }
                }
                "blue" => {
                    if count > max_value_of_blue {
                        max_value_of_blue = count;
                    }
                }
                _ => (),
            }
        }
    }

    println!("Game id: {}, ", game_id);
    println!("Max value of red: {}, ", max_value_of_red);
    println!("Max value of green: {}, ", max_value_of_green);
    println!("Max value of blue: {}, ", max_value_of_blue);

    let result = max_value_of_red * max_value_of_green * max_value_of_blue;

    println!("Result: {}", result);

    (result)
}

fn main() {
    let mut sum_of_game_ids = 0;
    let mut sum_of_power_of_colors = 0;

    match read_file() {
        Ok(lines) => {
            for line in lines {
                let (game_id, results) = get_results_for_game(line.clone());
                let power_of_colors = get_power_of_colors(line.clone());
                sum_of_power_of_colors += power_of_colors;

                if results.iter().any(|&x| x == false) {
                    continue;
                } else {
                    sum_of_game_ids += game_id;
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }

    println!("Sum of game ids: {}", sum_of_game_ids);
    println!("Sum of power of colors: {}", sum_of_power_of_colors);
}
