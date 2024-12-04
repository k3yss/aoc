use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let problem_part = &args[2];
    let file = File::open(file_path).expect("Filepath is invalid");
    let reader = BufReader::new(file);

    if problem_part == "1" {
        let result = corrupt_multiplication(reader);
        println!("Result is {}", result);
    } else if problem_part == "2" {
        let result = corrupt_multiplication_with_switch(reader);
        println!("Result is {}", result);
    }
}

fn multiplication(i: usize, list_of_characters: Vec<char>) -> i32 {
    let mut result = 0;
    let mut first_digit = String::new();
    let mut second_digit = String::new();
    let mut pos = i + 4; // Start after the '('

    while let Some(&ch) = list_of_characters.get(pos) {
        if ch.is_digit(10) {
            first_digit.push(ch);
            pos += 1;
        } else if ch == ',' {
            pos += 1;
            break;
        } else {
            first_digit = "0".to_string();
            second_digit = "0".to_string();
            break;
        }
    }

    while let Some(&ch) = list_of_characters.get(pos) {
        if ch.is_digit(10) {
            second_digit.push(ch);
            pos += 1;
        } else if ch == ')' {
            break;
        } else {
            // Invalid character found
            first_digit = "0".to_string();
            second_digit = "0".to_string();
            break;
        }
    }

    if let (Ok(first), Ok(second)) = (first_digit.parse::<i32>(), second_digit.parse::<i32>()) {
        result += first * second;
    }
    return result;
}

fn corrupt_multiplication(reader: BufReader<File>) -> i32 {
    let mut result: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let list_of_characters: Vec<char> = line.chars().collect();
            let iter = list_of_characters.windows(4);
            for (i, window) in iter.enumerate() {
                if window == ['m', 'u', 'l', '('] {
                    result += multiplication(i, list_of_characters.clone())
                    // Now you can parse the number
                }
            }
        } else {
            eprintln!("Error reading line");
        }
    }
    return result;
}

fn corrupt_multiplication_with_switch(reader: BufReader<File>) -> i32 {
    let mut multiplication_switch = true;

    let mut result: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let list_of_characters: Vec<char> = line.chars().collect();
            let iter = list_of_characters.windows(4);

            for (i, window) in iter.enumerate() {
                if window == ['m', 'u', 'l', '('] && multiplication_switch {
                    result += multiplication(i, list_of_characters.clone())
                } else if window == ['d', 'o', '(', ')'] {
                    multiplication_switch = true;
                } else if window == ['d', 'o', 'n', '\''] {
                    let t_position = i + 4;
                    let left_bracket_position = i + 5; // Start after the '('
                    let right_bracket_position = i + 6; // Start after the '('
                    if list_of_characters.get(t_position) == Some(&'t') && list_of_characters.get(left_bracket_position) == Some(&'(')
                        && list_of_characters.get(right_bracket_position) == Some(&')')
                    {
                        multiplication_switch = false;
                    }
                }
            }
        } else {
            eprintln!("Error reading line");
        }
    }
    return result;
}
