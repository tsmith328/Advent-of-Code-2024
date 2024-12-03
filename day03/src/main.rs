use std::fs;
use std::str;
use regex::Regex;

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

fn parse_input(input: &str) -> String {
    let mut file_contents = fs::read_to_string(input).expect("File should exist.");
    file_contents = file_contents.replace("\r","");
    file_contents = file_contents.replace("\n", "");
    return file_contents;
}

fn part1(input: String) -> i32 {
    let pattern = r"mul\(\d\d?\d?,\d\d?\d?\)";
    let mut total = 0;
    let re = Regex::new(pattern).unwrap();
    let hits = re.find_iter(&input);

    for hit in hits {
        let hit_str = hit.as_str();
        //println!("{hit_str}");
        let hit_str = hit_str.replace("mul(", "");
        let hit_str = hit_str.replace(")","");
        let mut operands: Vec<&str> = hit_str.split(",").collect();
        let left_operand = i32::from_str_radix(operands.pop().unwrap(), 10).unwrap();
        let right_operand = i32::from_str_radix(operands.pop().unwrap(), 10).unwrap();
        total += left_operand * right_operand;
    }
 
    total
}

fn part2(input: String) -> i32 {
    let pattern = r"mul\(\d\d?\d?,\d\d?\d?\)|do\(\)|don't\(\)";
    let mut total = 0;
    let re = Regex::new(pattern).unwrap();
    let hits = re.find_iter(&input);
    let mut active = true;

    for hit in hits {
        let hit_str = hit.as_str();
        //println!("{hit_str}");
        if hit_str == r"do()" {
            active = true;
        } else if hit_str == r"don't()" {
            active = false;
        } else if active {
            let hit_str = hit_str.replace("mul(", "");
            let hit_str = hit_str.replace(")","");
            let mut operands: Vec<&str> = hit_str.split(",").collect();
            let left_operand = i32::from_str_radix(operands.pop().unwrap(), 10).unwrap();
            let right_operand = i32::from_str_radix(operands.pop().unwrap(), 10).unwrap();
            total += left_operand * right_operand;
        }
    }
 
    total
}
