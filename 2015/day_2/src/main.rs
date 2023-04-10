use std::fs::File;
use std::io::prelude::*;

fn calculate_rectangle_area_and_add_additional_area(width: u32, height: u32, length: u32) -> u32 {
    let dimension_1 = width * height;
    let dimension_2 = width * length;
    let dimension_3 = height * length;

    let vector_of_dimensions = vec![dimension_1, dimension_2, dimension_3];
    let smallest_dimension = vector_of_dimensions.iter().min().unwrap();

    return 2 * (dimension_1 + dimension_2 + dimension_3) + smallest_dimension;
}

fn calculate_total_ribbon_length(width: u32, height: u32, length: u32) -> u32 {
    let mut vector_of_dimensions = vec![width, height, length];

    vector_of_dimensions.sort();

    let smallest_dimension = vector_of_dimensions[0];
    let second_smallest_dimension = vector_of_dimensions[1];

    return 2 * (smallest_dimension + second_smallest_dimension) + width * height * length;
}

fn read_file() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    return data;
}

fn main() {
    let data = read_file();

    let array_of_dimensions: Vec<&str> = data.split("\n").collect();
    let mut total_area_of_wrap_paper: u32 = 0;

    array_of_dimensions.iter().for_each(|dimension| {
        let array_of_dimensions: Vec<&str> = dimension.split("x").collect();

        if array_of_dimensions.len() != 3 {
            return;
        }

        let width = array_of_dimensions[0].parse::<u32>().unwrap();
        let height = array_of_dimensions[1].parse::<u32>().unwrap();
        let length = array_of_dimensions[2].parse::<u32>().unwrap();

        let total_area = calculate_rectangle_area_and_add_additional_area(width, height, length);

        total_area_of_wrap_paper += total_area;
    });

    let mut total_ribbon_length: u32 = 0;

    array_of_dimensions.iter().for_each(|dimension| {
        let array_of_dimensions: Vec<&str> = dimension.split("x").collect();

        if array_of_dimensions.len() != 3 {
            return;
        }

        let width = array_of_dimensions[0].parse::<u32>().unwrap();
        let height = array_of_dimensions[1].parse::<u32>().unwrap();
        let length = array_of_dimensions[2].parse::<u32>().unwrap();

        let total_ribbon_length_for_present = calculate_total_ribbon_length(width, height, length);

        total_ribbon_length += total_ribbon_length_for_present;
    });

    println!("Total area of wrap paper: {}", total_area_of_wrap_paper);
    println!("Total ribbon length: {}", total_ribbon_length);
}
