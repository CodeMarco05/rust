use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./src/input.txt").expect("Failed to read file");
    let reader = BufReader::new(file);
    let mut substrings: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let content = line.expect("Error in line");
        let index = content.find(":").expect("No index found");
        let content = content[index + 1..].to_string();
        let content = remove_whitespace(&content);

        // Split by semicolons and convert to strings
        let split_result: Vec<String> = content.split(';').map(|s| s.to_string()).collect();
        substrings.push(split_result);
    }

    // Print the result
    // for (index, slice) in substrings.iter().enumerate() {
    //     println!("Slice at index {}: {:?}", index, slice);
    // }

    for (i, row) in substrings.iter().enumerate(){
        for(j, string) in row.iter().enumerate(){
            println!("Row: {} Element: {}", i, string);
        }
    }



}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}
