use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let file_path = "input.txt";

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    // Parse the file into my character grid.
    for line in reader.lines() {
        grid.push(line.unwrap().trim().chars().collect());
    }

    let part1_total = part_one(grid.clone());
    let part2_total = part_two(grid.clone());
    println!("Part 1 total: {}", part1_total);
    println!("Part 2 total: {}", part2_total);
}

fn part_one(grid: Vec<Vec<char>>) -> u32 {
    let mut total: u32 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            let max_len = row.len() - 1;
            let max_height = grid.len() - 1;

            if ch.is_numeric() {
                // Check to see if we've already computed this number.
                if x > 0 {
                    if row[x - 1].is_numeric() {
                        continue;
                    }
                }

                // Get the full number
                let full_number = extract_number(x, row.clone());
                let index = x + full_number.to_string().len();

                // Build an array of all surrounding values.
                let mut surrounding_vals: Vec<char> = Vec::new();
                let left_bound;
                let right_bound;
                if x > 0 {
                    left_bound = x - 1;
                    surrounding_vals.push(row[x - 1]);
                } else {
                    left_bound = 0
                }
                if index <= max_len {
                    right_bound = index;
                    surrounding_vals.push(row[index]);
                } else {
                    right_bound = max_len;
                }

                for r in left_bound..=right_bound {
                    if y > 0 {
                        surrounding_vals.push(grid[y - 1][r]);
                    }
                    if y < max_height {
                        surrounding_vals.push(grid[y + 1][r]);
                    }
                }

                // Check if anything in the array is a symbol
                for character in surrounding_vals {
                    if character != '.' {
                        total += full_number
                    }
                }
            }
        }
    }

    return total;
}

fn part_two(grid: Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    let max_height = grid.len() - 1;
    for (y, row) in grid.iter().enumerate() {
        let max_len = row.len() - 1;
        for (x, ch) in row.iter().enumerate() {
            if *ch == '*' {
                // keep track of all the surrounding numbers we find
                let mut gear_pieces: Vec<u32> = Vec::new();

                // Look left
                if x > 0 && row[x - 1].is_numeric() {
                    gear_pieces.push(extract_number(x - 1, row.clone()));
                }

                // Look right
                if x < max_len && row[x + 1].is_numeric() {
                    gear_pieces.push(extract_number(x + 1, row.clone()));
                }

                // Look above
                if y > 0 {
                    // Check directly above. If this is a number, it must be the only one above this symbol.
                    if grid[y - 1][x].is_numeric() {
                        gear_pieces.push(extract_number(x, grid[y - 1].clone()));
                    } else {
                        // Check above to the left
                        if x > 0 && grid[y - 1][x - 1].is_numeric() {
                            gear_pieces.push(extract_number(x - 1, grid[y - 1].clone()));
                        }
                        // Check above to the right
                        if x < max_len && grid[y - 1][x + 1].is_numeric() {
                            gear_pieces.push(extract_number(x + 1, grid[y - 1].clone()));
                        }
                    }
                }

                // Look below
                if y < max_height {
                    // Check directly above. If this is a number, it must be the only one above this symbol.

                    if grid[y + 1][x].is_numeric() {
                        gear_pieces.push(extract_number(x, grid[y + 1].clone()));
                    } else {
                        // Check above to the left
                        if x > 0 && grid[y + 1][x - 1].is_numeric() {
                            gear_pieces.push(extract_number(x - 1, grid[y + 1].clone()));
                        }
                        // Check above to the right
                        if x < max_len && grid[y + 1][x + 1].is_numeric() {
                            gear_pieces.push(extract_number(x + 1, grid[y + 1].clone()));
                        }
                    }
                }

                // Check to see if I have the right number of gear pieces:
                if gear_pieces.len() == 2 {
                    // Add the product of the two pieces to the total.
                    total += gear_pieces[0] * gear_pieces[1];
                }
            }
        }
    }
    return total;
}

// Given a vector of characters and a start position, it will extract the number
// at that spot (going out in either direction), and return the index of the starting spot.
fn extract_number(start_position: usize, line: Vec<char>) -> u32 {
    let mut true_start_position = start_position;
    while true_start_position > 0 && line[true_start_position - 1].is_numeric() {
        true_start_position -= 1;
    }

    let max_len = line.len();
    let mut index = true_start_position;
    let mut full_number = 0;
    while index < line.len() && index < max_len {
        match line[index].to_digit(10) {
            Some(numeral) => {
                full_number = full_number * 10 + numeral;
                index += 1;
            }
            None => {
                break;
            }
        }
    }

    return full_number;
}
