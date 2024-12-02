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

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let file_contents = fs::read_to_string(input).expect("File should exist.");
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in file_contents.lines(){
        let levels = line.split_whitespace().map(|x| i32::from_str_radix(x, 10).unwrap()).collect();
        reports.push(levels);
    }

    reports
}

fn part1(_: Vec<Vec<i32>>) -> i32 {
    0
}

fn part2(_: Vec<Vec<i32>>) -> i32 {
    0
}
