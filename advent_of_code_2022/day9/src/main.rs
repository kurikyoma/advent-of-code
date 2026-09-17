use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!(
        "The number of visited locations with a short tail is {}.",
        tail_position_counter(2)
    ); // answer for my input: 6023

    println!(
        "The number of visited locations with a long tail is {}.",
        tail_position_counter(10)
    ); // answer for my input: 2533
}

fn tail_position_counter(rope_len: i32) -> usize {
    let file_path =
        r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day9\my_test.txt";
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut rope: Vec<Position> = Vec::new();
    for _ in 0..rope_len {
        rope.push(Position { x: 0, y: 0 });
    }
    let mut visited_locations = HashMap::new();
    visited_locations.insert(Position { x: 0, y: 0 }, true);

    for line in contents.split("\n") {
        let v: Vec<&str> = line.trim().split(" ").collect();
        let count: i32 = v[1].parse().unwrap();
        let direction;
        //println!("Number of steps to take is {}", count);

        use Direction::*;
        match v[0].chars().nth(0).unwrap() {
            'U' => direction = Up,
            'D' => direction = Down,
            'L' => direction = Left,
            'R' => direction = Right,
            _ => panic!("Invalid direction in input."),
        }

        for _ in 0..count {
            get_new_position(&mut rope[0], &direction);

            for i in 1..rope.len() {
                let leader = rope[i-1];
                if !new_spot(&mut rope[i], leader) {
                    break;
                }
            }
            visited_locations.insert(*rope.last().unwrap(), true);
        }

        //println!("Just completed this step: {}", line);
        //println!("Rope now: {:?}", rope);
        //println!("The number of visited locations after that step is {}.", visited_locations.keys().len());
        //println!("Rope Position: {:?}", rope);
    }

    return visited_locations.keys().len();
}

fn get_new_position(current_spot: &mut Position, direction: &Direction) {
    use Direction::*;
    match direction {
        Up => current_spot.y += 1,
        Down => current_spot.y -= 1,
        Left => current_spot.x -= 1,
        Right => current_spot.x += 1,
    };
}

fn new_spot(follower: &mut Position, leader: Position) -> bool {

    if follower.x - leader.x > 1 || leader.x - follower.x > 1 {
        follower.y = leader.y;
        if leader.x > follower.x {
            follower.x += 1;
        } else {
            follower.x -= 1;
        }
        return true;
    }
    if follower.y - leader.y > 1 || leader.y - follower.y > 1 {
        follower.x = leader.x;
        if leader.y > follower.y {
            follower.y += 1;
        } else {
            follower.y -= 1;
        }
        return true;
    }
    return false;
}
