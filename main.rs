use std::collections::{HashMap, HashSet};
use std::fs;

// fn check_grid(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
//     let rows = grid.len() as i32;
//     let cols = grid[0].len() as i32;
//
//     let deltas = [
//         (-1, -1), (-1, 0), (-1, 1),
//         (0, -1), (0, 1),
//         (1, -1), (1, 0), (1, 1)
//     ];
//
//     for (dr, dc) in &deltas {
//         let new_row = row as i32 + dr;
//         let new_col = col as i32 + dc;
//         if 0 <= new_row && new_row < rows && 0 <= new_col && new_col < cols {
//             if grid[new_row as usize][new_col as usize] != '.' && !grid[new_row as usize][new_col as usize].is_digit(10) {
//                 return true;
//             }
//         }
//     }
//
//     return false;
// }
//
// fn main() {
//     let file = fs::read_to_string("input.txt").expect("can parse");
//     let grid: Vec<Vec<char>> = file
//         .lines()
//         .map(|l| l.chars().collect())
//         .collect();
//
//     let mut bitmask: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
//     let mut sum = 0;
//
//     for (row_index, row) in grid.iter().enumerate() {
//         for (col_index, ch) in row.iter().enumerate() {
//             if ch.is_digit(10) && !bitmask[row_index][col_index] {
//                 let mut is_counted = false;
//                 // check if its digit, then it must be start of a number
//                 let mut num_sequence = String::new();
//                 let mut i = col_index;
//                 while i < row.len() && grid[row_index][i].is_digit(10) {
//                     if check_grid(&grid, row_index, i) {
//                         is_counted = true;
//                     }
//                     num_sequence.push(grid[row_index][i]);
//                     bitmask[row_index][i] = true;
//                     i += 1;
//                 }
//
//                 let number = num_sequence.parse::<i32>().unwrap();
//                 if is_counted {
//                     sum += number;
//                 }
//             }
//         }
//     }
//     println!("{:?}", sum);
// }

// fn main() {
//     // Mutable vector declaration using 'mut'
//     let my_vector = vec![1, 2, 3];
//
//     // Modifying the vector - appending a new element
//     // my_vector.push(4);
//
//     // Changing an element at index 1
//     my_vector[1] = 5;
//
//     println!("{:?}", my_vector); // Output: [1, 5, 3, 4]
// }

fn get_gears(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<(i32, i32)> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let deltas = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    let mut gears = Vec::new();

    for (dr, dc) in &deltas {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        if 0 <= new_row && new_row < rows && 0 <= new_col && new_col < cols {
            if grid[new_row as usize][new_col as usize] == '*' {
                gears.push((new_row, new_col));
            }
        }
    }

    gears
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");
    let grid: Vec<Vec<char>> = file
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut bitmask: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut sum = 0;

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, ch) in row.iter().enumerate() {
            if ch.is_digit(10) && !bitmask[row_index][col_index] {
                let mut relevant_gears: Vec<(i32, i32)> = Vec::new();
                // check if its digit, then it must be start of a number
                let mut num_sequence = String::new();
                let mut i = col_index;
                while i < row.len() && grid[row_index][i].is_digit(10) {
                    let rg = get_gears(&grid, row_index, i);
                    relevant_gears.extend(rg);
                    num_sequence.push(grid[row_index][i]);
                    bitmask[row_index][i] = true;
                    i += 1;
                }

                let final_gears: Vec<(i32, i32)> = relevant_gears.into_iter().collect::<HashSet<_>>().into_iter().collect();
                let number = num_sequence.parse::<i32>().unwrap();
                for (x, y) in final_gears {
                    let new_x = x as usize;
                    let new_y = y as usize;
                    let new_coord = (new_x, new_y);
                    if let Some(entry) = gears.get_mut(&new_coord) {
                        entry.push(number);
                    } else {
                        gears.insert(new_coord, vec![number]);
                    }
                }
            }
        }
    }

    for (_, value) in &gears {
        if value.len() == 2 {
            let ratio = value.first().unwrap() * value.last().unwrap();
            sum += ratio;
        }
    }
    println!("{:?}", sum);
}
