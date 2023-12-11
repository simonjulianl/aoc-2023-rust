// use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

use num::abs;

// fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
//     if matrix.is_empty() {
//         return vec![];
//     }
//
//     let num_rows = matrix.len();
//     let num_cols = matrix[0].len();
//
//     let mut result = vec![vec!['.'; num_rows]; num_cols];
//     for i in 0..num_rows {
//         for j in 0..num_cols {
//             result[j][i] = matrix[i][j];
//         }
//     }
//     result
// }

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Galaxy {
    row: usize,
    col: usize,
    scaled_row: usize,
    scaled_col: usize,
}
//
// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
// struct Pair {
//     galaxies: [Galaxy; 2],
//     shortest_distance: i64,
// }

fn main() {
    let file: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("can parse")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let expansion_factor = 1_000_000 - 1; // change for part 1 and 2
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for i in 0..file.len() {
        for j in 0..file[0].len() {
            if file[i][j] == '#' {
                let g = Galaxy { row: i, col: j, scaled_row: i, scaled_col: j };
                galaxies.push(g);
            }
        }
    }

    // part 2
    for (idx, row) in file.iter().enumerate() {
        if row.iter().all(|&s| s == '.') {
            // make every galaxy on the bottom shifted by expansion factor
            galaxies.iter_mut().filter(|s| s.row > idx).for_each(|s| s.scaled_row += expansion_factor);
        }
    }

    for col_nbr in 0..file[0].len() {
        let col: Vec<char> = file.iter().map(|s| s[col_nbr]).collect();
        if col.iter().all(|&s| s == '.') {
            galaxies.iter_mut().filter(|s| s.col > col_nbr).for_each(|s| s.scaled_col += expansion_factor);
        }
    }

    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let d = abs(g1.scaled_row as i64 - g2.scaled_row as i64) + abs(g1.scaled_col as i64 - g2.scaled_col as i64);
            ans += d;
        }
    }
    println!("{:?}", ans);
    // part 1
    // let mut new_map: Vec<Vec<char>> = Vec::new();
    // for row in &file {
    //     if row.iter().all(|&s| s == '.') {
    //         for _ in 0..expansion_factor {
    //             new_map.push(row.clone());
    //         }
    //     } else {
    //         new_map.push(row.clone());
    //     }
    // }
    //
    // let mut expanded_map: Vec<Vec<char>> = Vec::new();
    // for col_nbr in 0..new_map[0].len() {
    //     let col: Vec<char> = new_map.iter().map(|s| s[col_nbr]).collect();
    //     if col.iter().all(|&s| s == '.') {
    //         for _ in 0..expansion_factor {
    //             expanded_map.push(col.clone());
    //         }
    //     } else {
    //         expanded_map.push(col);
    //     }
    // }
    //
    // expanded_map = transpose(&expanded_map);
    //
    // let mut ans = HashSet::new();
    // for i in 0..expanded_map.len() {
    //     for j in 0..expanded_map[0].len() {
    //         if expanded_map[i][j] == '#' {
    //             let g = Galaxy { row: i, col: j };
    //             let pairs = bfs(&expanded_map, g);
    //             ans.extend(pairs);
    //         }
    //     }
    // }

    // let s = ans.iter().fold(0, |acc, &Pair { galaxies, shortest_distance }| acc + shortest_distance);


    // let test_galaxy = Galaxy { row: 11, col: 0 };
    // let ans = bfs(&expanded_map, test_galaxy);

    // println!("{:?}", s);
}

// part 1
// fn bfs(map: &Vec<Vec<char>>, source: Galaxy) -> HashSet<Pair> {
//     let mut distances: HashSet<Pair> = HashSet::new();
//     let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();
//     let mut visited: HashSet<(usize, usize)> = HashSet::new();
//     queue.push_back((source.row, source.col, 0));
//     visited.insert((source.row, source.col));
//
//     // println!("{:?}", map[0].len());
//     let max_row = map.len();
//     let max_col = map[0].len();
//
//     while !queue.is_empty() {
//         let (row, col, dist) = queue.pop_front().unwrap();
//         // row 2 col 0
//         if map[row][col] == '#' {
//             // new galaxy
//             let new_galaxy = Galaxy { row, col };
//             if new_galaxy != source {
//                 let mut galaxies = [source, new_galaxy];
//                 galaxies.sort();
//                 let np = Pair {
//                     galaxies,
//                     shortest_distance: dist,
//                 };
//                 distances.insert(np);
//             }
//         }
//         for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
//             let new_row = row as i64 + dx;
//             let new_col = col as i64 + dy;
//             if 0 <= new_row && new_row < max_row as i64 && 0 <= new_col && new_col < max_col as i64 {
//                 if !visited.contains(&(new_row as usize, new_col as usize)) {
//                     visited.insert((new_row as usize, new_col as usize));
//                     queue.push_back((new_row as usize, new_col as usize, dist + 1));
//                 }
//             }
//         }
//     }
//
//     // print!("{:?}", visited.len());
//     distances
// }