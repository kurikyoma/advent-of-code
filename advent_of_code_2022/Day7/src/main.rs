use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let file_path =
        r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day7\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut dir_map: HashMap<String, u64> = HashMap::new();
    dir_map.insert(String::from("/"), 0);
    let mut current_path = Vec::new();
    let mut reading_dir = false;
    let mut folder_size = 0;

    for line in contents.split("\n") {
        let ln = line.trim();

        // This kind of line represents a user command.
        if ln.chars().nth(0).unwrap() == '$' {
            // If we were reading a directory, we've now hit the end.
            if reading_dir == true {
                let mut key: String = String::new();
                for dir in current_path.clone() {
                    if key.len() == 0 {
                        key = format!("{}", dir);    
                    }
                    else {
                        key = format!("{}.{}", key, dir);
                    }
                    *dir_map.get_mut(&key).unwrap() += folder_size;
                }
                folder_size = 0;
                reading_dir = false;
            }

            // This means they're gathering data about the current dir.
            if ln.contains("$ ls") {
                // We don't have to read it if we already know what's in it.
                if *dir_map.get(&current_path.join(".")).unwrap() == 0 {
                    reading_dir = true;
                }
            }

            if ln.contains("$ cd") {
                if ln.contains("..") {
                    current_path.pop();
                } else {
                    let k = String::from(ln.split(" ").last().unwrap());
                    current_path.push(k.clone());
                    dir_map.insert(current_path.join("."), 0);
                }
            }
        }

        if reading_dir == true {
            let size_str = ln.split(" ").nth(0).unwrap();
            let size = match u64::from_str(size_str) {
                Ok(val) => val,
                Err(_) => 0,
            };
            folder_size += size;
        }
    }

    // If the last step happened in the middle of reading, we have to
    // be sure to include it as well.
    if reading_dir == true {
        let mut key: String = String::new();
        for dir in current_path.clone() {
            if key.len() == 0 {
                key = format!("{}", dir);    
            }
            else {
                key = format!("{}.{}", key, dir);
            }
            *dir_map.get_mut(&key).unwrap() += folder_size;
        }
    }

    let mut sizes: Vec<u64> = dir_map.values().cloned().collect();
    sizes.sort();

    let mut total_size: u64 = 0;
    let mut smallest_size = 0;
    let size_needed = 30000000 - (70000000 - sizes.last().unwrap());
    for val in sizes {
        if val <= 100000 {
            total_size += val;
        }
        if val >= size_needed && smallest_size == 0 {
            smallest_size = val;
        }

    }

    println!("The total size is: {}", total_size);
    println!("The smallest size dir is {}", smallest_size);
}
