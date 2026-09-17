use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug)]
struct NoValueErr {
    message: String,
}

// Implement the Display trait for the error
impl fmt::Display for NoValueErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no valid value on line: {}", self.message)
    }
}

// Implement the Error trait for the error
impl error::Error for NoValueErr {}

fn main() {
    // Replace "your_file.txt" with the path to your file
    let file_path = "input.txt";

    println!("Part 1 Value: {}", part_one(file_path));
    println!("Part 2 Value: {}", part_two(file_path));
}

fn part_one(file_path: &str) -> i32 {
        // Open the file
        let file = File::open(file_path).unwrap();
        let reader = io::BufReader::new(file);
        let mut total = 0;
    
        // Iterate over lines in the file
        for line in reader.lines() {
            // Handle each line as needed
            let valid_line = line.unwrap();

            let val = get_numerics_and_positions(valid_line);
            match val {
                Ok(numbers) => {
                    let (tens, _, ones, _) = numbers;
                    let value = (10*tens) + ones;
                    total += value;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        return total
}


fn part_two(file_path: &str) -> i32 {
    // Open the file
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let valid_line = line.unwrap();

        let tens;
        let ones;

        let nums_val = get_numerics_and_positions(valid_line.clone());
        match nums_val {
            // there is at least one numeric digit in the string
            Ok(contents) => {
                let (tens_num, tens_num_pos, ones_num, ones_num_pos) = contents;
                let strings_val = get_string_vals_and_positions(valid_line.clone());
                match strings_val {
                    // there is at least one number word in the string
                    Ok(contents) => {
                        let (tens_str, tens_str_pos, ones_str, ones_str_pos) = contents;
                        if tens_num_pos < tens_str_pos {
                            tens = tens_num;
                        } else {
                            tens = tens_str;
                        }
                
                        if ones_num_pos > ones_str_pos {
                            ones = ones_num;
                        } else {
                            ones = ones_str;
                        }
                    }
                    // there are no number words in the string
                    Err(_) => {
                        tens = tens_num;
                        ones = ones_num;
                    }
                }
            }
            // there are no numeric digits in the string
            Err(_) => {
                (tens, _, ones, _) = get_string_vals_and_positions(valid_line.clone()).unwrap();
            }
        }
    
        let value = (10*tens) + ones;
        //println!("{}", value);
        total += value;
    }

    return total;
}

fn get_numerics_and_positions(string: String) -> Result<(i32, usize, i32, usize), NoValueErr> {
    let mut tens: i32 = -1;
    let mut ones: i32 = -1;

    let mut tens_pos = 0;
    let mut ones_pos = 0;

    let mut pos: usize = 0;

    for c in string.chars() {
        // Attempt to convert the character to an integer
        let val = c.to_digit(10);
        match val {
            Some(x) => {
                if tens < 0 {
                    tens = x.try_into().unwrap();
                    tens_pos = pos;
                }
                ones = x.try_into().unwrap();
                ones_pos = pos;
            },
            None => ()
        }
        pos += 1;
    }

    if tens < 0 || ones < 0 {
        return Err(NoValueErr {
            message: string.to_string(),
        })
    }
    return Ok((tens, tens_pos, ones, ones_pos))
}

fn get_string_vals_and_positions(string: String) -> Result<(i32, usize, i32, usize), NoValueErr> {
    let number_map: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    let num_arr = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut tens: i32 = -1;
    let mut ones: i32 = -1;

    let mut tens_pos = string.len() + 1;
    let mut ones_pos = 0;

    for number in num_arr {
        for (index, _) in string.match_indices(number) {
            if index < tens_pos {
                tens_pos = index;
                tens = number_map[number]
            }
            if index >= ones_pos {
                ones_pos = index;
                ones = number_map[number];
            }
        }
    }

    if tens < 0 || ones < 0 {
        return Err(NoValueErr {
            message: string.to_string(),
        })
    }

    return Ok((tens, tens_pos, ones, ones_pos))
}