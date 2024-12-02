use std::fs;
use std::str;


const TEST_FILE: &str = "input_test.txt";
const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut part1_output = part1(parse_input(TEST_FILE));
    println!("Part 1 Test ouput: {part1_output}");
    part1_output = part1(parse_input(INPUT_FILE));
    println!("Part 1 Output: {part1_output}");
    
    println!("---------------------------------------");

    let mut part2_output = part2(parse_input(TEST_FILE));
    println!("Part 2 Test ouput: {part2_output}");
    part2_output = part2(parse_input(INPUT_FILE));
    println!("Part 2 Output: {part2_output}");
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let file_contents = fs::read_to_string(input).expect("File should exist.");
    let mut values1: Vec<i32> = Vec::new();
    let mut values2: Vec<i32> = Vec::new();

    for line in file_contents.lines(){
        let mut items = line.split_whitespace();
        let item_left = items.next().expect("Each line should contain two items separated by whitespace.");
        let item_right = items.next().expect("Each line should contain two items separated by whitespace.");

        let item1_value = i32::from_str_radix(item_left, 10).expect("Item values should be integers.");
        let item2_value = i32::from_str_radix(item_right, 10).expect("Item values should be integers.");

        values1.push(item1_value);
        values2.push(item2_value);
    }

    (values1, values2)
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut values1 = input.0;
    let mut values2 = input.1;
    let mut total: i32 = 0;

    values1.sort();
    values2.sort();

    for i in 0..values1.len(){
        let mut diff = values1[i] - values2[i];
        if diff < 0 {
            diff *= -1;
        }
        total += diff;
    }

    return total;
}

fn part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut similarity = 0;

    let input_left = input.0;
    let input_right = input.1;

    for val in input_left {
        let num_matches = input_right.iter().filter(|&n| *n == val).count();
        similarity += val * num_matches as i32;
    }

    return similarity;
}