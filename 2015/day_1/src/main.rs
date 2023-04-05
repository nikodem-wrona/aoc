use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut floor = 0;

    data.chars().enumerate().for_each(|(i, c)| {    
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if (floor == -1) {
            println!("Basement: {}", i + 1);
        }
    });

    println!("Floor: {}", floor);
}
