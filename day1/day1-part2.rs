use std::fs;
use std::io::{self, prelude::*, BufReader};
use fs::File;
use std::collections::HashMap;

// create a map with values from first list and zero values initially after which interate the
// second array and increment the value array if key is found

fn main() -> io::Result<()> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut list1: HashMap<i32, i32> = HashMap::new();
    let mut list2: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let numbers: Vec<&str> = line.split_whitespace().collect();

            if let Some(first) = numbers.first() {
                if let Ok(num) = first.parse::<i32>() {
                        list1.entry(num).or_insert(0);
                        let count = list2.entry(num).or_insert(0);
                        *count += 1;
                }
            }
        }
    }

    // Second pass: Open the file again and increment values
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            if let Some(last) = numbers.last() {
                if let Ok(num) = last.parse::<i32>() {
                    if let Some(count) = list1.get_mut(&num) {
                        *count += 1;
                    }
                }
            }
        }
    }

    let mut result: i32 = 0;

    for (key, value) in list1 {
        let multiplication_value = list2.get(&key).copied().unwrap_or(0);
        result += key*value*multiplication_value;
    }

    println!("{:?}", result);
    Ok(())
}

