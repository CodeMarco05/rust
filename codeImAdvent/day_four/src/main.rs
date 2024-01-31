use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Error in reading file");
    let reader = BufReader::new(file);

    let mut lucky_numbers: Vec<String> = Vec::new();
    let mut input_numbers: Vec<String> = Vec::new();

    for line in reader.lines() {
        let content = line.expect("Error in reading line.");
        let index_colon = content.find(":").expect("Colon not found");
        let content = content[index_colon + 2..].to_string();
        let index_divider = content.find("|").expect("| not found");
        let lucky_numbers_string = content[..index_divider - 1].to_string();
        let input_numbers_string = content[index_divider + 2..].to_string();
        lucky_numbers.push(lucky_numbers_string);
        input_numbers.push(input_numbers_string);
    }
    let mut lucky_numbers_indexed: Vec<Vec<String>> = Vec::new();
    for line in lucky_numbers.iter() {
        let mut splitet: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        splitet.retain(|e| !e.is_empty());

        lucky_numbers_indexed.push(splitet);
    }

    let mut input_numbers_indexed: Vec<Vec<String>> = Vec::new();
    for line in input_numbers.iter() {
        let mut splitet: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        splitet.retain(|e| !e.is_empty());

        input_numbers_indexed.push(splitet);
    }

    let input_numbers: Vec<Vec<i32>> = input_numbers_indexed
        .iter()
        .map(|line| {
            line.iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let lucky_numbers: Vec<Vec<i32>> = lucky_numbers_indexed
        .iter()
        .map(|line| {
            line.iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let sum = matches_for_2d_array(input_numbers, lucky_numbers);
    println!("Result: {}", sum);
}

fn matches_for_2d_array(mut input_numbers: Vec<Vec<i32>>, mut lucky_numbers: Vec<Vec<i32>>) -> i32 {
    if !input_numbers.is_empty() || !lucky_numbers.is_empty() {
        let mut sum = 0;

        let mut input_numbers_to_compute: Vec<Vec<i32>> = Vec::new();
        let mut lucky_numbers_to_compute: Vec<Vec<i32>> = Vec::new();

        for (i, line) in lucky_numbers.iter().enumerate() {
            let elements_to_check = input_numbers[i].clone();
            let matches = count_matches(elements_to_check, line.clone());
            sum+=1;
            for indices in 1..=matches {
                let u_size_index: usize = indices as usize;
                let element_to_push_input = input_numbers[i + u_size_index].clone();
                input_numbers_to_compute.push(element_to_push_input);

                let element_to_push_lucky = lucky_numbers[i + u_size_index].clone();
                lucky_numbers_to_compute.push(element_to_push_lucky);
            }
        }

        println!("Lucky: {:?}", lucky_numbers_to_compute);
        println!("Input: {:?}", input_numbers_to_compute);

        return sum + matches_for_2d_array(input_numbers_to_compute, lucky_numbers_to_compute);
    }
    return 0;

}

fn count_matches(input_elements: Vec<i32>, line: Vec<i32>) -> i32 {
    let mut right_elements = 0;
    for element in input_elements.iter() {
        if line.contains(element) {
            right_elements += 1;
        }
    }
    return right_elements;
}
