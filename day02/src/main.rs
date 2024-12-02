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

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    
    for report in input {
        if report_is_safe(report.clone()) {
            safe_reports += 1;
        }
    }

    return safe_reports;
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for report in input {
        if report_is_safe(report.clone()) {
            safe_reports += 1;
        } else if dampener(report){
            safe_reports += 1;
        }
    }
    return safe_reports;
}

fn report_is_safe(report: Vec<i32>) -> bool {
    let next_levels = report.iter().skip(1);
        let deltas: Vec<i32> = report.iter().zip(next_levels).map(|(curr, next)| next - curr).collect();

        let gaps_ok = deltas.iter()
                        .fold(true, |acc, x| acc && ((x > &0 && x <= &3) || (x < &0 && x >= &-3)));
        
        if !gaps_ok { return false; }

        let all_positive = deltas.iter().all(|x| x > &0);
        if all_positive {
            return true;
        }
        let all_negative = deltas.iter().all(|x| x < &0);
        if all_negative {
            return true;
        }
        return false;
}

fn dampener(report: Vec<i32>) -> bool {
    for test_idx in 0..report.len() {
        let mut test_report = report.clone();
        test_report.remove(test_idx);

        if report_is_safe(test_report) {
            return true;
        }
    }

    return false;
}