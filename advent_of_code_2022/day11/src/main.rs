use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    operator: u64,
    test_divisor: u64,
    true_index: usize,
    false_index: usize,
    counter: u64,
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Multiply,
    Subtract,
    Square,
    Double
}

impl Monkey {
    fn new() -> Monkey {
        return Monkey{items: vec![], operation: Operation::Add, operator: 0, test_divisor: 0, true_index: 0, false_index: 0, counter: 0};
    }

    fn operate(&self, a: u64) -> u64 {
        match self.operation {
            Operation::Add => self.add(a),
            Operation::Multiply => self.multiply(a),
            Operation::Subtract => self.subtract(a),
            Operation::Square => self.square(a),
            Operation::Double => self.double(a),
        }
    }

    fn do_test(&self, item: u64) -> bool {
        return item % self.test_divisor == 0;
    }
    
    fn add(&self, a: u64) -> u64 {
        return a + self.operator;
    }

    fn multiply(&self, a: u64) -> u64 {
        return a * self.operator;
    }

    fn subtract(&self, a: u64) -> u64 {
        return a - self.operator;
    }

    fn square(&self, a: u64) -> u64 {
        return a * a;
    }

    fn double(&self, a: u64) -> u64 {
        return a + a;
    }

    fn receive_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day11\input.txt";
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut monkey_array: Vec<Monkey> = Vec::new();
    for line in contents.split("\n") {
        if line.contains("Monkey") {
            monkey_array.push(Monkey::new());
        }
        if line.contains("Starting items:") {
            let starting_items: Vec<&str> = line.split(":").last().unwrap().trim().split(",").collect();
            for item in starting_items {
                // Parse the item into a number and then add it to the monkey's queue.
                monkey_array.last_mut().unwrap().receive_item(item.trim().parse::<u64>().unwrap());
            }
        }
        if line.contains("Operation:") {
            let things: Vec<&str> = line.trim().split(" ").collect();
            let monkey = monkey_array.last_mut().unwrap();
            
            // Get the operator.
            let o = things.len() - 2;
            match things[o].chars().collect::<Vec<char>>()[0] {
                '+' => monkey.operation = Operation::Add,
                '*' => monkey.operation = Operation::Multiply,
                '-' => monkey.operation = Operation::Subtract,
                _ => panic!("Invalid Operator.")
            };
            
            let num_str = things.last().unwrap().trim();

            if num_str.contains("old") {
                if monkey.operation == Operation::Add {
                    monkey.operation = Operation::Double;                    
                }
                else {
                    monkey.operation = Operation::Square;
                }
            }
            else {
                monkey.operator = num_str.parse::<u64>().unwrap();
            }
        }
        if line.contains("Test: divisible") {
            monkey_array.last_mut().unwrap().test_divisor = line.trim().split(" ").collect::<Vec<&str>>().last().unwrap().trim().parse::<u64>().unwrap();
        }
        if line.contains("If true:") {
            monkey_array.last_mut().unwrap().true_index = line.trim().split(" ").collect::<Vec<&str>>().last().unwrap().trim().parse::<usize>().unwrap();
        }
        if line.contains("If false:") {
            monkey_array.last_mut().unwrap().false_index = line.trim().split(" ").collect::<Vec<&str>>().last().unwrap().trim().parse::<usize>().unwrap();
        }
    }

    let mut ultimate_divisor: u64 = 1;
    for m in &monkey_array {
        ultimate_divisor = ultimate_divisor * m.test_divisor;
    }


    // Do that monkey math.
    //for _ in 0..20 { // part 1
    for _ in 0..10000 { // part 2
        for i in 0..monkey_array.len() {
            while monkey_array[i].items.len() > 0 {
                let mut item = monkey_array[i].items.remove(0);
                item = monkey_array[i].operate(item);
                // item = item % 3  // part 1
                item = item % ultimate_divisor; // part 2
                if monkey_array[i].do_test(item) {
                    let index = monkey_array[i].true_index;
                    monkey_array[index].items.push(item);
                } else {
                    let index = monkey_array[i].false_index;
                    monkey_array[index].items.push(item);
                }
                monkey_array[i].counter += 1;
            }
        }
    }

    let mut biggest: u64 = 0;
    let mut second_biggest = 0;
    for monkey in &monkey_array {
        if monkey.counter > biggest {
            second_biggest = biggest;
            biggest = monkey.counter;
        }
        else if monkey.counter > second_biggest {
            second_biggest = monkey.counter;
        }
    }

    for (i, m) in monkey_array.iter().enumerate() {
        println!("Monkey {}: {:?}", i, m);
    }

    println!("Your monkey business score is: {}", biggest * second_biggest);
}
