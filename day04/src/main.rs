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

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut return_val = vec![vec!['0';0];0];
    let file_contents = fs::read_to_string(input).expect("File should exist.");
    let lines: Vec<&str> = file_contents.lines().collect();
    for line in lines {
        return_val.push(line.chars().collect());
    }
    
    return return_val;
}

fn part1(input: Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, curr_char) in line.iter().enumerate() {
            if *curr_char == 'X' {
                // Check all 8 ways "XMAS" could go:
                // We don't need to check bounds if the checker function returns 0 gracefully...
                
                // Moving up and to the left
                total += check_xmas(&input, x, y, -1, -1);
                // Moving to the left
                total += check_xmas(&input, x, y, -1, 0);
                // Moving down and to the left
                total += check_xmas(&input, x, y, -1, 1);
                // Moving down
                total += check_xmas(&input, x, y, 0, 1);
                // Moving down and right
                total += check_xmas(&input, x, y, 1, 1);
                // Moving right
                total += check_xmas(&input, x, y, 1, 0);
                // Moving up and right
                total += check_xmas(&input, x, y, 1, -1);
                // Moving up
                total += check_xmas(&input, x, y, 0, -1);
            }
        }
    }

    total
}

fn check_xmas(input: &Vec<Vec<char>>, curr_x: usize, curr_y: usize, dx: i32, dy: i32) -> i32 {
    let height = input.len();
    let width = input[0].len();

    // Check bounds. If dx or dy is negative, we have to be 3+ positions away from the edge on that dimension.
    // e.g. 0 1 2 3 4 5
    //      S A M X
    //          X M A S

    if dx < 0 && curr_x <= 2 {
        return 0;
    }

    if dy < 0 && curr_y <= 2 {
        return 0;
    }

    if dx > 0 && curr_x >= (width - 3) {
        return 0;
    }

    if dy > 0 && curr_y >= (height - 3) {
        return 0;
    }

    // Now we can check that each character is what it should be.
    // We know Char 1 is 'X'
    let mut x = ((curr_x as i32) + dx) as usize;
    let mut y = ((curr_y as i32) + dy) as usize;
    // Is Char 2 'M'?
    if input[y][x] != 'M' {
        return 0;
    }
    // Is Char 3 'A'?
    x = ((x as i32) + dx) as usize;
    y = ((y as i32) + dy) as usize;
    if input[y][x] != 'A' {
        return 0;
    }

    // Is Char 4 'S'?
    x = ((x as i32) + dx) as usize;
    y = ((y as i32) + dy) as usize;
    if input[y][x] != 'S' {
        return 0;
    }

    1
}

fn part2(input: Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, curr_char) in line.iter().enumerate() {
            if *curr_char == 'A' {
                // 'A' will always be in the center of the cross-MAS.
                // Need to check top left, top right, bottom left, bottom right characters
                
                let height = input.len();
                let width = input[0].len();
                
                // If we are on the outer edge of the crossword, we cannot find a match
                if x == 0 || y == 0 || x == (width - 1) || y == (height - 1) {
                    continue;
                }
                
                let top_left = input[y-1][x-1];
                let top_right = input[y-1][x+1];
                let bottom_left = input[y+1][x-1];
                let bottom_right = input[y+1][x+1];

                // Check that TL is 'M' and BR is 'S' or TL is 'S' and BR is 'M'
                if (top_left == 'M' && bottom_right == 'S')
                    || (top_left == 'S' && bottom_right == 'M') {
                        // Check that TR is 'M and BL is 'S' or vice-versa
                        if (top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M') {
                            total += 1;
                    }
                }
            }
        }
    }

    total
}