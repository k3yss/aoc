//  their list of locations to check is currently empty.
//
//
//  Eventually, someone decides that the best place to check first would be the Chief Historian's office.
//
//  the historically significant locations are listed not by name but by a unique number called the location ID.
//
//  Sort both the array, absolute minus the value add them
//
//

use std::fs;
use std::io::{self, prelude::*, BufReader};
use fs::File;

fn main() -> io::Result<()> {
    //let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        // first element of the line append to list1
        // last element of the line to list2
        let line = line?;
        if !line.is_empty() {
            let numbers: Vec<&str> = line.split_whitespace().collect();

            if let Some(first) = numbers.first() {
                if let Ok(num) = first.parse::<i32>() {
                    list1.push(num);
                }
            }
            if let Some(last) = numbers.last() {
                if let Ok(num) = last.parse::<i32>() {
                    list2.push(num);
                }
            }
        }
    }
    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
        println!("Both the arrays are of different lenghts");
    }

    let mut result: i32 = 0;

    for i in 0..list1.len() {
        result += (list1[i] - list2[i]).abs();
    }
    println!("{:?}", result);
    Ok(())
}

