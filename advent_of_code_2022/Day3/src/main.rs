use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day3\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents.clone();
    get_split_priority(contents.clone());
    get_trio_priority(contents);
}

fn get_priority(l: char) -> u32 {
    let val = l as u32;
    let mut ret = 0;
    if val >= 97 {
        ret = val - 'a' as u32 + 1;
    }
    else {
        ret = val - 'A' as u32 + 27;
    }

    return ret;
}

fn get_split_priority (contents: String) {
    let mut total_priority: u32 = 0;

    for line in contents.split("\n") {
        let ln = line.trim();

        let (split1, split2) = ln.split_at(ln.len()/2);

        let compartment1: HashSet<char> = split1.chars().collect();
        let mut common_letters: Vec<char> = Vec::new();
        for letter in split2.chars() {
            if compartment1.contains(&letter) {
                if !common_letters.contains(&letter) {
                    common_letters.push(letter);
                }
            }
        }

        for l in common_letters.iter() {
             total_priority += get_priority(*l);
        }
    }

    println!("Total Priority: {}", total_priority);
}

fn get_trio_priority(contents: String) {
    let mut total_priority: u32 = 0;
    let mut trio_set: Vec<&str> = Vec::new();

    for line in contents.split("\n") {
        let ln = line.trim();
        trio_set.push(ln);
        
        if trio_set.len() == 3 {
            total_priority += get_this_set_priority(trio_set);
            trio_set = Vec::new();
        }
    }

    println!("Total Priorty for the trios: {}", total_priority);
}

fn get_this_set_priority(set: Vec<&str>) -> u32 {
    let mut collection_vec: Vec<HashSet<char>> = Vec::new();
    for elf in set {
        collection_vec.push(elf.chars().collect());
    }

    let mut common_vec: Vec<char> = Vec::new();
    let mut in_all_3 = false;
    for l in &collection_vec[0] {
        for item in &collection_vec {
            if item.contains(l) {
                in_all_3 = true;
            }
            else {
                in_all_3 = false;
                break;
            }
        }

        if in_all_3 {
            common_vec.push(*l);
        }
    }

    if common_vec.len() != 1 {
        panic!("Too many common items found: {:?}.", common_vec)
    }
    else {
        return get_priority(common_vec[0]);
    }

}