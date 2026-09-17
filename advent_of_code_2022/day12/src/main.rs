use std::cmp::Ordering;
use std::{collections::HashMap, fs, hash::Hash};

#[derive(Debug, Eq, Clone, Copy)]
struct Node {
    location: Point,
    height: u32,
    shortest_path: u64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.shortest_path.cmp(&other.shortest_path)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
            && self.height == other.height
            && self.shortest_path == other.shortest_path
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let file_path =
        r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day12\input.txt";
    //println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut unvisited_locations: Vec<Node> = Vec::new();
    let mut starting_point: Point = Point { x: 0, y: 0 };
    let mut starting_options: Vec<Point> = vec![];
    let mut ending_point: Point = Point { x: 0, y: 0 };

    for (y, line) in contents.split("\n").enumerate() {
        let ln = line.trim();
        for (x, point) in ln.chars().enumerate() {
            if point == 'S' {
                unvisited_locations.push(Node {
                    location: Point { x: x as i32, y: y as i32},
                    height: 'a' as u32,
                    shortest_path: u64::MAX,
                });
                starting_point = unvisited_locations.last().unwrap().location;
                starting_options.push(starting_point);
            } 
            else if point == 'E' {
                unvisited_locations.push(Node {
                    location: Point { x: x as i32, y: y as i32},
                    height: 'z' as u32,
                    shortest_path: u64::MAX,
                });
                ending_point = unvisited_locations.last().unwrap().location;
            }
            else if point == 'a' {
                unvisited_locations.push(Node {
                    location: Point { x: x as i32, y: y as i32},
                    height: point as u32,
                    shortest_path: u64::MAX,
                });
                starting_options.push(unvisited_locations.last().unwrap().location);
            }
            else {
                unvisited_locations.push(Node {
                    location: Point { x: x as i32, y: y as i32},
                    height: point as u32,
                    shortest_path: u64::MAX,
                });
            }
        }
    }
    println!("The shortest path is this long: {}", get_shortest_path(unvisited_locations.clone(), starting_point, ending_point));

    let mut absolute_shortest = u64::MAX;
    //println!("Starting options: {:?}", starting_options);
    for sp in starting_options {
        let length = get_shortest_path(unvisited_locations.clone(), sp, ending_point);
        if length < absolute_shortest {
            absolute_shortest = length;
        }
    }
    
    println!("The shortest possible path with a new starting point is: {}", absolute_shortest);
}

fn get_shortest_path(
    unvisited_locations: Vec<Node>,
    starting_point: Point,
    ending_point: Point,
) -> u64 {
    let mut current_point = starting_point.clone();
    let mut neighbor_finder = HashMap::new();
    let mut neighbor;
    let mut neighborly_array = vec![];
    
    for spot in unvisited_locations {
        let mut new_spot = spot.clone();
        if spot.location == starting_point {
            new_spot.shortest_path = 0;
            neighborly_array.push(new_spot.clone());
        }
        neighbor_finder.insert(spot.location, new_spot);
    }
    

    while neighborly_array.len() > 0 {
        current_point = neighborly_array.remove(0).location;
        let x = current_point.x;
        let y = current_point.y;
        if !neighbor_finder.contains_key(&current_point) {
            continue;
        }
        if current_point == ending_point {
            break;
        }
        let c = neighbor_finder.remove(&current_point).unwrap();
        //println!("Current point = {:?}", c); 
        if c.shortest_path == u64::MAX {
            // This means there is no path.
            return u64::MAX;
        }
        let neighborray = vec![Point { x: x - 1, y: y }, Point { x: x, y: y - 1 }, Point { x: x + 1, y: y }, Point { x: x, y: y + 1 }];
        for p in neighborray {
            if neighbor_finder.contains_key(&p) {
                neighbor = neighbor_finder.remove(&p).unwrap();
                if neighbor.height <= c.height + 1 && neighbor.shortest_path > c.shortest_path {
                    neighbor.shortest_path = c.shortest_path + 1;
                    neighborly_array.push(neighbor.clone());
                }
                neighbor_finder.insert(neighbor.location, neighbor);
            }
        }

        neighborly_array.sort_unstable();

        //println!("{:?}", n);
        //println!("n0: {:?}", n[0]);

    }
    let final_destination: &Node;
    if neighbor_finder.contains_key(&current_point) {
        final_destination = neighbor_finder.get(&current_point).unwrap();
        //println!("{:?}", final_destination);
        return final_destination.shortest_path;
    }
    else {
        return u64::MAX;
    }
}
