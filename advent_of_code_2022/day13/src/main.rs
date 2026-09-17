use std::fs;

#[derive(Debug)]
struct Lister {
    numbers: Vec<i32>,
    depth: i32,
    order: u32,
}

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day13\example.txt";
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut ln1 = "";
    let mut ln2 = "";
    for line in contents.split("\n") {
        let mut lister_list1: Vec<Lister> = vec![];
        let mut lister_list2: Vec<Lister> = vec![];
        if ln1.len() == 0 {
            ln1 = line.trim();
        }
        else if ln2.len() == 0 {
            ln2 = line.trim();
            let mut current_depth = -1;
            let mut order = 0;
            for part in ln1.split(",") {
                let mut tmp: String = "".to_string();
                for c in part.chars() {
                    if c == '[' {
                        current_depth += 1;
                        order += 1;
                        lister_list1.push(Lister{numbers: vec![], depth: current_depth, order: order});
                    }
                    else if c == ']' {
                        if tmp.len() > 0 {
                            lister_list1[current_depth as usize].numbers.push(tmp.parse::<i32>().unwrap());
                            tmp = "".to_string();
                        }
                        current_depth -= 1;
                    }
                    else {
                        tmp.push(c);
                    }
                }
                if tmp.len() > 0 {
                    lister_list1.last_mut().unwrap().numbers.push(tmp.parse::<i32>().unwrap());
                    tmp = "".to_string();
                }
            }
    
            if current_depth != -1 {
                panic!("We didn't close all of our loops.")
            }
    
            println!("ln1: {:?}", lister_list1);
        }
        else if line.trim().len() == 0 {
            ln1 = "";
            ln2 = "";
        }
    }
}
