use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day1\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    top_elf(contents.clone());
    top_three(contents);
}

fn top_elf(contents: String) {
    let mut max: u64 = 0;
    let mut current: u64 = 0;
    for line in contents.split("\n") {
        let ln = line.trim();
        if ln.len() > 0 {
            println!("current line: '{}'", ln);
            current += u64::from_str(ln).unwrap();
        }
        else {
            if current > max {
                max = current;
            }
            current = 0;
        }
    }

    println!("The elf with the most calories has {} calories of snacks.", max);
}

fn top_three(contents: String) {
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut third: u64 = 0;

    let mut current: u64 = 0;
    for line in contents.split("\n") {
        let ln = line.trim();
        if ln.len() > 0 {
            println!("current line: '{}'", ln);
            current += u64::from_str(ln).unwrap();
        }
        else {
            if current > first {
                third = second;
                second = first;
                first = current;
            }
            else if current > second {
                third = second;
                second = current;
            }
            else if current > third {
                third = current;
            }
            current = 0;
        }
    }

    println!("The top 3 elves have a total of {} calories worth of snacks.", first + second + third);
}