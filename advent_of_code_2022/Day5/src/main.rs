use std::fs;
use math::round;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day5\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).unwrap();

    let mut phase = 1;
    let mut array_of_columns: Vec<Vec<char>> = Vec::new();

    for i in 0..9 {
        array_of_columns.push(Vec::new());
    }

    for line in contents.split("\n") {        
        
        // Don't need to count the column labels.
        if line.trim().len() != 0 && line.chars().nth(1).unwrap() == '1' {
            phase = 2;
            continue;
        }

        // For the blank dividing line
        if line.trim().len() == 0 {
            continue;
        }

        // Step 1: Generate the current state.
        if phase == 1 {
            let mut index = 0;
            while index < line.len() {
                if index % 4 == 1 {
                    let stack_letter = line.chars().nth(index).unwrap();
                    if !stack_letter.is_whitespace() {
                        let col_num = math::round::floor((index as f64)/4.0, 0) as usize;
                        array_of_columns[col_num].insert(0, stack_letter);
                    }
                }

                index += 1;
            }
            continue;
        }

        // Step 2: Parse moves.
        let mut parts = line.split_whitespace();
        let mut info: Vec<usize> = Vec::new();
        for item in parts {
            let val = match item.parse::<usize>() {
                Ok(num) => num,
                Err(e) => 0 
            };

            if val != 0 {
                info.push(val);
            }
        }

        // Cratemover 9000
        /*
        for i in 0..info[0] {
            let moving_value = array_of_columns[info[1]-1].pop().unwrap();
            array_of_columns[info[2]-1].push(moving_value);
        }
        */

        // Cratemover 9001
        let mut holding_pen: Vec<char> = Vec::new();
        for i in 0..info[0] {
            let moving_value = array_of_columns[info[1]-1].pop().unwrap();
            holding_pen.push(moving_value);
        }

        while holding_pen.len() > 0 {
            array_of_columns[info[2]-1].push(holding_pen.pop().unwrap());
        }


    }

    for i in 0..9 {
        println!("{:?}", array_of_columns[i]);
    }

}
