use std::fs;

fn main() {
    let file_path =
        r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day10\input.txt";
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut cycle_count = 1;
    let mut x = 1;
    let mut total_power = 0;
    let mut image: String = "".to_string();
    for line in contents.split("\n") {
        let ln = line.trim();
        let words: Vec<&str> = ln.split(" ").collect();
        let add_cycles: i32;
        let update_power: i32;
        
        // no-op case
        if words.len() == 1 {
            add_cycles = 1;
            update_power = 0;
        } 
        // addx case
        else {
            update_power = words[1].parse().unwrap();
            add_cycles = 2;
        }

        for _ in 0..add_cycles {
            //println!("At cycle {} x is {}", cycle_count, x);
            let pos = (cycle_count - 1) % 40;
            if pos == 0 {
                image.push_str("\n");
            }
            if x + 1 == pos || x - 1 == pos || x == pos {
                image.push_str("#");
            }
            else {
                image.push_str(".");
            }
            if cycle_count % 40 == 20 {
                total_power += cycle_count * x;
                //println!("Total power is now: {}", total_power);
            }
            cycle_count += 1;
        }

        x += update_power;
    }

    println!("Total power: {}", total_power);

    println!("The image: {}", image);
}
