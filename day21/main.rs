use std::collections::{HashSet, VecDeque};

use num::traits::Euclid;

type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn translate(&self, map: &Matrix<char>) -> Position {
        let row_ln = map.len();
        let col_ln = map[0].len();
        Position {
            row: self.row.rem_euclid(row_ln as i64),
            col: self.col.rem_euclid(col_ln as i64),
        }
    }

    fn get_next(&self, map: &Matrix<char>) -> Vec<Position> {
        let mut result = Vec::new();
        let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        for (dy, dx) in &directions {
            let new_position = Position {
                row: self.row + dy,
                col: self.col + dx,
            };

            // part 1
            // if 0 <= new_row && new_row < map.len() as i64 && 0 <= new_col && new_col < map[0].len() as i64 {
            //     if map[new_row as usize][new_col as usize] == '.' || map[new_row as usize][new_col as usize] == 'S' {
            //         result.push(Position {
            //             row: new_row as usize,
            //             col: new_col as usize,
            //         });
            //     }
            // }

            // part 2
            let translated_position = new_position.translate(map);
            let translated_char = map[translated_position.row as usize][translated_position.col as usize];
            match translated_char {
                'S' | '.' => {
                    result.push(new_position);
                }
                _ => (),
            }
        }

        result
    }
}

fn main() {
    // let x = -1;
    // println!("{:?}", x.rem_euclid(&10));
    let input = include_str!("input.txt");
    let parsed_inputs: Matrix<char> = input.lines().map(|s| s.chars().collect()).collect();

    // get initial position
    let mut init_position = Position { row: 0, col: 0 };
    'outer: for i in 0..parsed_inputs.len() {
        for j in 0..parsed_inputs[0].len() {
            if parsed_inputs[i][j] == 'S' {
                init_position = Position {
                    row: i as i64,
                    col: j as i64,
                };
                break 'outer;
            }
        }
    }
    let mut q = VecDeque::new();
    q.push_back(init_position);
    println!("Starting position: {:?}", init_position);
    let total_step = 65 + 131 * 0;

    let xln = parsed_inputs[0].len();
    let yln = parsed_inputs.len();
    println!("Input size: {yln} x {xln}"); // square of 131 x 131
    println!("remainder of the square: {:?}", total_step % xln); // remainder is 65
    // return;

    let mut hs = HashSet::new();
    for _ in 0..total_step {
        hs.clear();
        let mut new_queue = VecDeque::new();
        while let Some(x) = q.pop_front() {
            let results = x.get_next(&parsed_inputs);
            for item in results {
                if !hs.contains(&item) {
                    hs.insert(item);
                    new_queue.push_back(item);
                }
            }
            // new_queue.append(&mut VecDeque::from(results));
        };
        q = new_queue;
    }

    // sorry today part 2 is too GG, I need to check reddit comments
    // and apparently there is an interesting observation, especially from charr3
    // apparently if f(n) = the number of spaces you can reach after n steps and X = grid.len(),
    // f(n), f(n + X), f(n + 2x) ... forms a quadratic equation (why and how?? I don't fully understand it yet)
    // since 26501365 = 202300 * 131 + 65, we try to find
    // f(65) = 3944
    // f(65 + 131) = 35082
    // f(65 + 131 * 2) = 97230
    // if f(n) = ax^2 + bx + c, using wolfram alpha we can get a = 15505/17161, b = 32273/17161, c = 76614/17161
    let a: f64 = 15505.0 / 17161.0;
    let b: f64 = 32273.0 / 17161.0;
    let c: f64 = 76614.0 / 17161.0;
    let total_step = 26501365.0;
    let result = total_step * total_step * a + total_step * b + c; // this is freaking black magic bro
    println!("{:?}", result);
}