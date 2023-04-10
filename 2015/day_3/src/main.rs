use std::fs::File;
use std::io::prelude::*;

struct Position {
    x: i32,
    y: i32,
}

fn read_file() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    return data;
}

fn get_all_positions(data: &String) -> Vec<(i32, i32)> {
    let mut position = Position { x: 0, y: 0 };
    let mut visited = vec![(0, 0)];

    for c in data.chars() {
        match c {
            '^' => position.y += 1,
            'v' => position.y -= 1,
            '>' => position.x += 1,
            '<' => position.x -= 1,
            _ => (),
        }

        visited.push((position.x, position.y));
    }

    return visited;
}

fn get_positions(data: &String, even: bool) -> Vec<(i32, i32)> {
    let mut position = Position { x: 0, y: 0 };
    let mut visited = vec![(0, 0)];

    for (index, c) in data.chars().enumerate() {
        let condition = if even { index % 2 == 0 } else { index % 2 != 0 };
        
        if !condition {
            continue;
        }

        match c {
            '^' => position.y += 1,
            'v' => position.y -= 1,
            '>' => position.x += 1,
            '<' => position.x -= 1,
            _ => (),
        }

        visited.push((position.x, position.y));
    }

    return visited;
}

fn get_santa_positions(data: &String) -> Vec<(i32, i32)> {
    return get_positions(data, false);
}

fn get_robo_santa_positions(data: &String) -> Vec<(i32, i32)> {
    return get_positions(data, true);
}

fn main() {
    let data = read_file();

    // Solve part 1
    let visited = get_all_positions(&data);

    let mut number_of_houses_with_multiple_visits = 0;

    let mut sorted_and_filtered = visited.clone();

    sorted_and_filtered.sort();
    sorted_and_filtered.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1); // remove duplicates based on the tuple values


    for (x, y) in sorted_and_filtered.iter() {
        let count = visited.iter().filter(|&&i| i == (*x, *y)).count();

        if count >= 1 {
            number_of_houses_with_multiple_visits += 1;
        }
    }
    
    println!(
        "Number of houses with multiple visit: {}",
        number_of_houses_with_multiple_visits
    );

    // Solve part 2
    let santa_visited = get_santa_positions(&data);
    let robo_santa_visited = get_robo_santa_positions(&data);

    let mut all_visited = santa_visited.clone();
    all_visited.append(&mut robo_santa_visited.clone());
    
    let mut sorted_and_filtered = all_visited.clone();
   
    sorted_and_filtered.sort();
    sorted_and_filtered.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1); // remove duplicates based on the tuple values

    let mut number_of_houses_with_multiple_visits = 0;
    
    for (x, y) in sorted_and_filtered.iter() {
        let count = all_visited.iter().filter(|&&i| i == (*x, *y)).count();

        if count >= 1 {
            number_of_houses_with_multiple_visits += 1;
        }
    }

    println!(
        "Number of houses with multiple visit: {}",
        number_of_houses_with_multiple_visits
    );
    
}
