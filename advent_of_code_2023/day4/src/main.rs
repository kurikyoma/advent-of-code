use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "input.txt";

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    // Parse the file into my character grid.
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    let part1_total = part1(lines.clone());
    let part2_total = part2(lines);

    println!("Part 1 total: {}", part1_total);
    println!("Part 2 total: {}", part2_total);
}

fn part1(lines: Vec<String>) -> u32 {
    let mut total = 0;

    // Parse the file into my character grid.
    for line in lines {
        let (_, winning_nums, my_nums) = extract_numbers_from_line(&line);

        total += get_score(winning_nums, my_nums);
    }

    return total;
}

fn part2(lines: Vec<String>) -> u32 {
    let mut total = 0;

    let mut card_map: HashMap<usize, u32> = HashMap::new();

    // Give every card a baseline value of 1
    for index in 0..lines.len() {
        card_map.insert(index, 1);
    }

    for index in 0..lines.len() {
        let (_, winning_nums, my_nums) = extract_numbers_from_line(&lines[index]);
        let count = get_matching_num_count(winning_nums, my_nums);
        for val in 1..=count {
            let current_val = *card_map.get(&(index + val as usize)).unwrap();
            let current_card_count = *card_map.get(&(index)).unwrap();
            card_map.insert(index + val as usize, current_val + current_card_count);
        }
    }

    for count in card_map.values() {
        total += *count;
    }

    return total;
}

fn extract_numbers_from_line(line: &str) -> (&str, Vec<&str>, Vec<&str>) {
    let mut initial_array: Vec<&str> = line.split(":").collect();
    let card_array: Vec<&str> = initial_array[0]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();
    let card = card_array[1];

    initial_array = initial_array[1].split("|").collect();
    let winning_nums: Vec<&str> = initial_array[0]
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();
    let my_nums: Vec<&str> = initial_array[1]
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();

    return (card, winning_nums, my_nums);
}

fn get_score(winning_numbers: Vec<&str>, my_numbers: Vec<&str>) -> u32 {
    let mut score = 0;
    let mut mymap: HashMap<&str, bool> = HashMap::new();
    for num in winning_numbers {
        mymap.insert(num, true);
    }

    let mut match_vector: Vec<&str> = Vec::new();

    for num in my_numbers {
        match mymap.get(num) {
            Some(_) => {
                match_vector.push(num);
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
            None => (),
        }
    }

    return score;
}

fn get_matching_num_count(winning_numbers: Vec<&str>, my_numbers: Vec<&str>) -> u32 {
    let mut count = 0;
    let mut mymap: HashMap<&str, bool> = HashMap::new();
    for num in winning_numbers {
        mymap.insert(num, true);
    }

    for num in my_numbers {
        match mymap.get(num) {
            Some(_) => {
                count += 1;
            }
            None => (),
        }
    }

    return count;
}
