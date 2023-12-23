use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

use num::abs;

// use crate::Direction::{Down, Left, Right, Up};

type Matrix<T> = Vec<Vec<T>>;
type Adjlist = HashMap<(usize, usize), Vec<(usize, usize)>>;

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// enum Direction {
//     Right,
//     Left,
//     Up,
//     Down,
// }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    Path,
    Forest,
    // Slope(Direction),
}

impl From<u8> for Block {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Forest,
            _ => Self::Path,
            // part 2
            // b'>' => Self::Slope(Right),
            // b'<' => Self::Slope(Left),
            // b'^' => Self::Slope(Up),
            // b'v' => Self::Slope(Down),
            // _ => panic!("oh no"),
        }
    }
}

fn parse(s: &str) -> Vec<Block> {
    s.bytes().map(Block::from).collect()
}

fn main() {
    let input = include_str!("input.txt");
    let map: Matrix<Block> = input.lines().map(parse).collect();
    let result = longest_path(&map);
    println!("{:?}", result);

    // let's use dfs first
}

fn longest_path(map: &Matrix<Block>) -> i64 {
    let rows = map.len();
    let cols = map[0].len();

    // part 2, create compressed graph of the intersections only
    let mut adjacency_list: Adjlist = HashMap::new();
    // let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // A block is an intersection if the block contains a turn
    // do edge compression
    for i in 0..rows {
        for j in 0..cols {
            let current_char = map[i][j];
            if current_char == Block::Path {
                // check neighbour in four direction

                if is_junction(map, i, j) {
                    let mut neighbours = Vec::new();

                    // find all the adjacent junctions
                    // left
                    for l in (0..j).rev() {
                        let c = map[i][l];
                        match c {
                            Block::Forest => break,
                            Block::Path => {
                                if is_junction(map, i, l) {
                                    neighbours.push((i, l));
                                    break;
                                }
                            }
                        }
                    }

                    // right
                    for r in j + 1..cols {
                        let c = map[i][r];
                        match c {
                            Block::Forest => break,
                            Block::Path => {
                                if is_junction(map, i, r) {
                                    neighbours.push((i, r));
                                    break;
                                }
                            }
                        }
                    }

                    // bottom
                    for b in i + 1..rows {
                        let c = map[b][j];
                        match c {
                            Block::Forest => break,
                            Block::Path => {
                                if is_junction(map, b, j) {
                                    neighbours.push((b, j));
                                    break;
                                }
                            }
                        }
                    }

                    // up
                    for u in (0..i).rev() {
                        let c = map[u][j];
                        match c {
                            Block::Forest => break,
                            Block::Path => {
                                if is_junction(map, u, j) {
                                    neighbours.push((u, j));
                                    break;
                                }
                            }
                        }
                    }
                    adjacency_list.insert((i, j), neighbours);
                }
            }
        }
    }

    // println!("{:?}", adjacency_list);
    let init_coord = (0, 1);
    let target = (map.len() - 1, map[0].len() - 2);
    let mut answer = 0;
    bfs(&adjacency_list, init_coord, target, &mut answer);
    // let (result, _) = dfs(map, &mut visited, 0, 1, 0);
    // result
    answer
}

fn is_junction(map: &Matrix<Block>, i: usize, j: usize) -> bool {
    let top_char = if i == 0 {
        None
    } else {
        map.get(i - 1).and_then(|r| r.get(j))
    };

    let bot_char = map.get(i + 1).and_then(|r| r.get(j));
    let left_char = if j == 0 {
        None
    } else {
        map.get(i).and_then(|r| r.get(j - 1))
    };
    let right_char = map.get(i).and_then(|r| r.get(j + 1));

    let is_junction = match (top_char, bot_char, left_char, right_char) {
        (Some(Block::Path), _, Some(Block::Path), _) |
        (Some(Block::Path), _, _, Some(Block::Path)) |
        (_, Some(Block::Path), Some(Block::Path), _) |
        (_, Some(Block::Path), _, Some(Block::Path)) |
        (Some(Block::Path), None, _, _) | (None, Some(Block::Path), _, _) | // terminal path
        (_, _, Some(Block::Path), None) | (_, _, None, Some(Block::Path)) => true,
        _ => false,
    };
    is_junction
}

// fn is_valid_move(map: &HashMap<>, new_row: i64, new_col: i64, dir: &Direction) -> bool {
//     let valid_coordinate = 0 <= new_row && new_row < map.len() as i64 && 0 <= new_col && new_col < map[0].len() as i64;
//     if valid_coordinate {
//         let new_char = map[new_row as usize][new_col as usize];
//         match (new_char, dir) {
//             (Block::Forest, _) => false,
//             (Block::Slope(Left), Right) | (Block::Slope(Right), Left) |
//             (Block::Slope(Up), Down) | (Block::Slope(Down), Up) => true,
//             _ => true
//         }
//     } else {
//         false
//     }
// }

fn bfs(map: &Adjlist, coord: (usize, usize), target: (usize, usize), length: &mut i64) {
    let mut queue = VecDeque::new();
    queue.push_back((vec![coord], 0));
    while let Some((history, current_distance)) = queue.pop_front() {
        let current_point = history.last().unwrap();
        let (row, col) = current_point;
        if *current_point == target {
            *length = max(*length, current_distance);
        }
        let neighbours = map.get(current_point).unwrap();
        for n in neighbours {
            let (nr, nc) = n;
            let distance = abs(*nr as i64 - *row as i64) + abs(*nc as i64 - *col as i64);
            if !history.contains(n) {
                let mut new_history = history.clone();
                new_history.push(*n);
                queue.push_back((new_history, current_distance + distance));
            }
        }
    }
    // println!("{row} {col} {length}");
    // if coord == target {
    //     *length = max(*length, curr_length);
    // }

    // if visited.contains(&coord) {
    //     return;
    // }

    // visited.insert(coord); // cannot visit again
    // let directions = [Right, Left, Up, Down];
    // let mut max_length = length;
    // let neighbours = map.get(&coord).unwrap();
    // for n in neighbours {
    // let new_row = row as i64 + match dir {
    //     Right | Left => 0,
    //     Up => -1,
    //     Down => 1,
    // };
    // let new_col = col as i64 + match dir {
    //     Up | Down => 0,
    //     Right => 1,
    //     Left => -1,
    // };

    // is valid
    // if visited.contains(n) {
    //     continue;
    // }
    // let (nr, nc) = n;
    // let distance = abs(*nr as i64 - row as i64) + abs(*nc as i64 - col as i64);
    // dfs(map, visited, *n, target, curr_length + distance, length);
    //
    // if is_valid_move(map, new_row, new_col, dir) {
    //     let (new_length, is_reaching_end) = dfs(map, visited, new_row as usize, new_col as usize, length + 1);
    //     if is_reaching_end {
    //         reach_end = true;
    //         max_length = max(max_length, new_length);
    //     }
    // }
    // }
    //
    // visited.remove(&coord);
}
