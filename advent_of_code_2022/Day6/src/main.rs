use std::fs;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day6\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).unwrap();

    println!("First unique 4 at index: {}", find_first_unique_n(&contents, 4));
    println!("First unique 14 at index: {}", find_first_unique_n(&contents, 14));
}

fn find_first_unique_n(contents: &String, number: usize) -> usize {
    let mut last_n: Vec<char> = Vec::new();
    for (i, letter) in contents.char_indices() {
        if i < number {
            last_n.push(letter);
            continue;
        }
        if has_dup(&last_n) {
            last_n.remove(0);
            last_n.push(letter)
        }
        else {
            return i;
        }
    }

    return 0;
}

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    return false;
}
