use std::collections::HashSet;
use std::fs;
use num::abs;

fn main() {
    let file: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("can parse")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let row_max: i64 = file.len() as i64;
    let col_max: i64 = file[0].len() as i64;
    // scan starting position
    let mut start: (i64, i64) = (0, 0);
    'outer: for (i, &ref row) in file.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = (i as i64, j as i64);
                break 'outer;
            }
        }
    }

    let mut ans: i64 = 0;
    let mut prev_pos = start;
    let mut curr_pos = start;

    fn get_diffs(c: &char) -> [(i64, i64); 2] {
        match c {
            '|' => [(-1, 0), (1, 0)],
            '-' => [(0, 1), (0, -1)],
            'L' => [(-1, 0), (0, 1)],
            'J' => [(-1, 0), (0, -1)],
            '7' => [(1, 0), (0, -1)],
            'F' => [(1, 0), (0, 1)],
            '.' => [(0, 0); 2],
            _ => panic!("oh no")
        }
    }

    // part 2, use ray tracing, I can try to use other methods later like Pick's theorem
    let mut loops: Vec<(i64, i64)> = Vec::new();
    loops.push(start);

    'outermost: loop {
        // check any connected pipe
        // println!("{:?}", curr_pos);
        if curr_pos == start {
            let diffs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            'outer: for (dy, dx) in diffs {
                let new_y = curr_pos.0 + dy;
                let new_x = curr_pos.1 + dx;
                if 0 <= new_y && new_y < row_max && 0 <= new_x && new_x < col_max {
                    let next_char = file[new_y as usize][new_x as usize];
                    let diffs = get_diffs(&next_char);
                    if diffs.iter().any(|&(dy, dx)| (new_y + dy, new_x + dx) == start) {
                        // this is the current
                        ans += 1;
                        curr_pos = (new_y, new_x);
                        loops.push(curr_pos);
                        break 'outer;
                    }
                }
            }
            // println!("{:?}", curr_pos);
        } else {
            // just traverse until you find the start again
            let curr_char = file[curr_pos.0 as usize][curr_pos.1 as usize];
            let diffs = get_diffs(&curr_char);
            for (dy, dx) in diffs {
                let new_y = curr_pos.0 + dy;
                let new_x = curr_pos.1 + dx;
                if (new_y, new_x) == prev_pos {
                    // println!("here");
                    continue;
                } else {
                    prev_pos = curr_pos;
                    curr_pos = (new_y, new_x);
                    loops.push(curr_pos);
                    ans += 1;
                    if curr_pos == start {
                        println!("{:?} here", ans / 2);
                        break 'outermost;
                    }
                    break;
                }
            }
        }
    }

    let mut area = 0;
    let loops_vec: Vec<(i64, i64)> = loops.iter().cloned().collect();
    // println!("{:?}", loops_vec);
    for i in 0..loops_vec.len() {
        let (a1, b1) = loops_vec[i];
        let (a2, b2) = loops_vec[(i + 1) % loops_vec.len()];
        area += (a1 * b2) - (a2 * b1);
    }

    area = abs(area) / 2;

    // picks theorem, A = i + b/2 - 1 hence i = A - b/2 + 1
    let r#final = area - ans / 2 + 1;
    println!("part2: {area} {ans} {final}");
    // let mut second_ans = 0;
    // for i in 0..file.len() {
    //     let mut current_state = false;
    //     let mut row_ans = 0;
    //     for j in 0..file[0].len() {
    //         let current_coordinate = (i as i64, j as i64);
    //         if loops.contains(&current_coordinate) {
    //             current_state = !current_state;
    //         } else {
    //             if current_state {
    //                 second_ans += 1;
    //                 row_ans += 1;
    //             }
    //         }
    //     }
    //     println!("{i} {row_ans}");
    // }
    // println!("{:?}", second_ans);
    // check any connected pipe from
}
//     // dfs, basically try to find the loop
//     let mut set: HashSet<(i64, i64)> = HashSet::new();
//     let mut stack: VecDeque<(i64, i64, i64)> = VecDeque::new();
//     stack.push_back(s);
//     // set.insert((s.0, s.1));
//
//     while !stack.is_empty() {
//         let (row, col, dist) = stack.pop_back().unwrap();
//
//         // check all the neighbours, where you can enter
//         let current_char = file[row as usize][col as usize];
//         if current_char == '.' {
//             // println!("{row} {col} {dist}");
//             continue;
//         }
//         // ans = max(ans, dist);
//         // distance[row as usize][col as usize] = dist;
//
//         match current_char {
//             'S' => {
//                 let dirs: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
//                 for (dx, dy) in dirs {
//                     let new_row = row + dx;
//                     let new_col = col + dy;
//                     if 0 <= new_row && new_row < row_max && 0 <= new_col && new_col < col_max {
//                         let next_char = file[new_row as usize][new_col as usize];
//                         if next_char != '.' {
//                             // set.insert((new_row, new_col));
//                             let new_item = (new_row, new_col, dist + 1);
//                             stack.push_back(new_item);
//                         }
//                     }
//                 }
//             }
//             '|' => {
//                 let mut is_found = push_top(&mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_bottom(row_max, &mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             '-' => {
//                 let mut is_found = push_left(&mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_right(col_max, &mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             'L' => {
//                 let mut is_found = push_top(&mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_right(col_max, &mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             'J' => {
//                 let mut is_found = push_top(&mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_left(&mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             '7' => {
//                 let mut is_found = push_left(&mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_bottom(row_max, &mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             'F' => {
//                 let mut is_found = push_right(col_max, &mut set, &mut stack, row, col, dist, &file);
//                 is_found = is_found | push_bottom(row_max, &mut set, &mut stack, row, col, dist, &file);
//                 if is_found {
//                     println!("{:?}", stack);
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     println!("{:?}", distance);
// }
//
// fn push_right(col_max: i64, set: &mut HashSet<(i64, i64)>, deque: &mut VecDeque<(i64, i64, i64)>, row: i64, col: i64, dist: i64, file: &Vec<Vec<char>>) -> bool {
//     if col + 1 < col_max {
//         let nc = (row, col + 1);
//         let new_item = (row, col + 1, dist + 1);
//         let next_char = file[row as usize][(col + 1) as usize];
//         if next_char == 'S' {
//             return true;
//         }
//         deque.push_back(new_item);
//         // if !set.contains(&nc) {
//         //     set.insert(nc);
//         // };
//     }
//
//     false
// }
//
// fn push_left(set: &mut HashSet<(i64, i64)>, deque: &mut VecDeque<(i64, i64, i64)>, row: i64, col: i64, dist: i64, file: &Vec<Vec<char>>) -> bool {
//     if col >= 1 {
//         let nc = (row, col - 1);
//         let new_item = (row, col - 1, dist + 1);
//         let next_char = file[row as usize][(col - 1) as usize];
//         if next_char == 'S' {
//             return true;
//         }
//         deque.push_back(new_item);
//         // if !set.contains(&nc) {
//             // set.insert(nc);
//         // }
//     }
//
//     false
// }
//
// fn push_bottom(row_max: i64, set: &mut HashSet<(i64, i64)>, deque: &mut VecDeque<(i64, i64, i64)>, row: i64, col: i64, dist: i64, file: &Vec<Vec<char>>) -> bool {
//     if row + 1 < row_max {
//         let nc = (row + 1, col);
//         let new_item = (row + 1, col, dist + 1);
//         let next_char = file[(row + 1) as usize][col as usize];
//         if next_char == 'S' {
//             return true;
//         }
//         deque.push_back(new_item);
//         // set.insert(nc);
//         // if !set.contains(&nc) {
//         // };
//     }
//
//     false
// }
//
// fn push_top(set: &mut HashSet<(i64, i64)>, deque: &mut VecDeque<(i64, i64, i64)>, row: i64, col: i64, dist: i64, file: &Vec<Vec<char>>) -> bool {
//     if row >= 1 {
//         let nc = (row - 1, col);
//         let new_item = (row - 1, col, dist + 1);
//         let next_char = file[(row - 1) as usize][col as usize];
//         if next_char == 'S' {
//             return true;
//         }
//         deque.push_back(new_item);
//         // if !set.contains(&nc) {
//             // set.insert(nc);
//         // }
//     }
//
//     false
// }
//
