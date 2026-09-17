use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Replace "your_file.txt" with the path to your file
    let file_path = "input.txt";

    let full_data = parser(file_path);

    // Part 1
    let mut game_counter: u32 = 0;
    let mut total: u32 = 0;
    for game in full_data.clone() {
        game_counter += 1;
        match game.get("red") {
            Some(x) => {
                if *x > 12 {
                    continue;
                }
            }
            None => (),
        }

        match game.get("green") {
            Some(x) => {
                if *x > 13 {
                    continue;
                }
            }
            None => (),
        }

        match game.get("blue") {
            Some(x) => {
                if *x > 14 {
                    continue;
                }
            }
            None => (),
        }

        //println!("adding game {}: {:?}", game_counter, game);

        total += game_counter;
    }
    println!("Part 1 total: {}", total);

    // Part 2
    let mut total_power: u32 = 0;
    for game in full_data {
        let mut power: u32 = 1;
        let values: Vec<_> = game.values().cloned().collect();
        for val in values {
            power = power * val;
        }

        total_power += power;
    }

    println!("Part 2 total: {}", total_power);
}

fn parser(path: &str) -> Vec<HashMap<String, u32>> {
    // Open the file
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_data_array: Vec<HashMap<String, u32>> = Vec::new();
    for line in reader.lines() {
        let mut color_counts: HashMap<String, u32> = HashMap::new();
        let valid_line = line.unwrap();
        // Split to get the game part and the data part
        let halves: Vec<&str> = valid_line.split(":").collect();

        // We're tracking the game part separately
        let handfuls: Vec<&str> = halves[1].split(";").collect();

        for handful in handfuls {
            let colors: Vec<&str> = handful.split(",").collect();

            for color in colors {
                let trimmed_color = color.trim();
                let values: Vec<&str> = trimmed_color.split(" ").collect();
                let num: u32 = values[0].parse::<u32>().unwrap();
                let c = values[1].to_owned();

                let existing_val = color_counts.get(&c);
                match existing_val {
                    Some(x) => {
                        if x < &num {
                            color_counts.insert(c, num);
                        }
                    }
                    None => {
                        color_counts.insert(c, num);
                    }
                }
            }
        }

        parsed_data_array.push(color_counts)
    }

    return parsed_data_array;
}
