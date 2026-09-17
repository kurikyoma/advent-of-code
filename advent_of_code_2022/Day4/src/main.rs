use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day4\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut count: u32 = 0;
    let mut count2: u32 = 0;
    for line in contents.split("\n") {
        let ln = line.trim();

        let mut spl = ln.split(",");

        let range1 = spl.next().unwrap();
        let range2 = spl.next().unwrap();

        let mut r1_spl = range1.split("-");
        let val1_lower = u32::from_str(r1_spl.next().unwrap()).unwrap();
        let val1_upper = u32::from_str(r1_spl.next().unwrap()).unwrap();
        
        let mut r2_spl = range2.split("-");
        let val2_lower = u32::from_str(r2_spl.next().unwrap()).unwrap();
        let val2_upper = u32::from_str(r2_spl.next().unwrap()).unwrap();

        if val1_lower <= val2_lower && val1_upper >= val2_upper {
            count += 1;
        }
        else if val2_lower <= val1_lower && val2_upper >= val1_upper {
            count += 1;
        }

        if (val1_lower <= val2_lower && val1_upper >= val2_lower) || (val1_lower <= val2_upper && val1_upper >= val2_upper) {
            count2 += 1;
        }
        else if (val2_lower <= val1_lower && val2_upper >= val1_lower) || (val2_lower <= val1_upper && val2_upper >= val1_upper) {
            count2 += 1;
        }
    }

    println!("The count of overlapping ranges is: {}", count);
    println!("The count of ranges with ANY overlap is: {}", count2);
}