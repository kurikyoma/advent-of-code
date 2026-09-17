use std::fs;

fn main() {
    let file_path = r"C:\Users\mcele\Documents\Coding Projects\advent_of_code\2022\Day8\input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let cont_iter: Vec<&str> = contents.split("\n").collect();
    let mut grid: Vec<Vec<u32>> = Vec::with_capacity(cont_iter.len());

    for line in cont_iter {
        let mut row: Vec<u32> = vec![];
        let ln = line.trim();
        for c in ln.chars() {
            match c.to_digit(10) {
                Some(o) => row.push(o),
                None => (),
            };
        }
        grid.push(row);
    }

    println!("The count is: {}", how_many_unblocked_trees(grid.clone()));
    println!("The highest treescore is: {}", highest_treescore(grid.clone()));
}

fn how_many_unblocked_trees(grid: Vec<Vec<u32>>) -> i32 {
    let mut view_counter = 0;

    for (i, vector) in grid.iter().enumerate() {
        let mut view = true;
        for (j, height) in vector.iter().enumerate() {
            if j == 0 || i == 0 || j == vector.len() || i == grid.len() {
                view_counter += 1;
                continue;
            }
            // from the top
            for x in 0..i {
                if grid[x][j] >= *height {
                    view = false;
                    //println!("Tree at [{}][{}] with height {} is blocked from the top.", i, j, *height);
                    break;
                }
            }
            if view == true {
                view_counter += 1;
                continue;
            }
            view = true;

            // from the bottom
            for x in i + 1..grid.len() {
                if grid[x][j] >= *height {
                    view = false;
                    //println!("Tree at [{}][{}] with height {} is blocked from the bottom.", i, j, *height);
                    break;
                }
            }
            if view == true {
                view_counter += 1;
                continue;
            }
            view = true;

            // from the left
            for x in 0..j {
                if grid[i][x] >= *height {
                    //println!("Tree at [{}][{}] with height {} is blocked from the left.", i, j, *height);
                    view = false;
                    break;
                }
            }
            if view == true {
                view_counter += 1;
                continue;
            }
            view = true;

            // from the right
            for x in j + 1..vector.len() {
                if grid[i][x] >= *height {
                    //println!("Tree at [{}][{}] with height {} is blocked from the right.", i, j, *height);
                    view = false;
                    break;
                }
            }
            if view == true {
                view_counter += 1;
                continue;
            }
            view = true;
        }
    }

    return view_counter;
}

fn highest_treescore(grid: Vec<Vec<u32>>) -> usize {
    let mut top_score = 0;

    for (i, vector) in grid.iter().enumerate() {
        let mut score = 1;
        for (j, height) in vector.iter().enumerate() {
            let mut top = 0;
            let mut bottom = 0;
            let mut left = 0;
            let mut right = 0;

            // from the top
            for x in (0..i).rev() {
                top += 1;
                if grid[x][j] >= *height {
                    break;
                }
            }

            // from the bottom
            for x in (i + 1)..grid.len() {
                bottom += 1;
                if grid[x][j] >= *height {
                    break;
                }
            }

            // from the left
            for x in (0..j).rev() {
                left += 1;
                if grid[i][x] >= *height {
                    break;
                }
            }

            // from the right
            for x in (j + 1)..vector.len() {
                right += 1;
                if grid[i][x] >= *height {
                    break;
                }
            }

            
            score = left * right * top * bottom;
            if score > top_score {
                //println!("New top score at [{}][{}]:", i, j);
                //println!("Left: {}, Right: {}, Top: {}, Bottom: {}, TOTAL: {}", left, right, top, bottom, score);
                top_score = score;
            }
        }
    }

    return top_score;
}