use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// This example data contains six reports each
// containing five levels.

// which reports are safe
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

// either all increasing or all decreasing

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("Enter file path");
    let file_path = &args[1];
    print!("Enter problem part");
    let problem_part = &args[2];
    println!("{}", file_path);
    let file = File::open(file_path).expect("Filepath is invalid");
    let reader = BufReader::new(file);

    if problem_part == "1" {
        let mut safe_reports = 0;

        for line in reader.lines() {
            safe_reports += calculate_safety(line.expect("Error "));
        }

        println!("Safe Reports are {:?}", safe_reports);
    } else {
        let mut safe_reports_after_tol = 0;

        for line in reader.lines() {
            safe_reports_after_tol += calculate_safety_after_tol(line.expect("Error "), false);
        }
        println!("Safe Reports after tol are {:?}", safe_reports_after_tol);
    }
}

fn calculate_safety(line: String) -> i32 {
    let parsed_integer_list: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse"))
        .collect();

    let mut is_negative = false;

    for i in 0..parsed_integer_list.len() - 1 {
        let mut sub_res = parsed_integer_list[i] - parsed_integer_list[i + 1];

        if i == 0 {
            if sub_res < 0 {
                is_negative = true;
            }
        }

        if (is_negative && sub_res > 0) || (!is_negative && sub_res < 0) {
            return 0;
        }

        sub_res = sub_res.abs();

        if !(1..=3).contains(&sub_res) {
            return 0;
        }
    }
    return 1;
}

fn calculate_safety_after_tol(line: String, first_pass_done: bool) -> i32 {
    let parsed_integer_list: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse"))
        .collect();

    if calculate_safety(line.clone()) == 1 {
        return 1;
    }

    if first_pass_done {
        return 0;
    }

    for i in 0..parsed_integer_list.len() {
        let mut new_list = parsed_integer_list.clone();
        new_list.remove(i);
        let new_list_str = new_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        // Check if removing this number makes the sequence safe
        if calculate_safety_after_tol(new_list_str, true) == 1 {
            return 1;
        }
    }
    return 0;
}
