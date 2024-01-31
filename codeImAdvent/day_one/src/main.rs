use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    const RADIX: u32 = 10;
    let mut sum: i32 = 0;

    let file = File::open("./src/input.txt").expect("Error when opening file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let content = line.expect("Error reading line");
        let mut digits: Vec<i8> = Vec::new();

        let content_array = content.char_indices();
        let mut storage = String::from("");

        for c in content_array {
            storage.push(c.1);
            println!("storage {}", storage);
            let result: (bool, u8) = check_for_number(&storage);
            if result.0 == true {
                println!("Number found: {}", result.1);
                break;
            }
        }

        //let content_array = content_array.rev();
    }

    println!("Sum of the calculation: {}", sum);
}

fn check_for_number(string: &String) -> (bool, u8) {
    let word_to_number: HashMap<&str, u8> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
    ]
    .into_iter()
    .collect();

    let numbers: HashMap<&str, u8> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("10", 10),
    ]
    .into_iter()
    .collect();

    for (word, number) in word_to_number {
        if string.contains(word) {
            return (true, number);
        }
    }

    return (false, 0);
}
