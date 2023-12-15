use std::collections::HashMap;

type Matrix<T> = Vec<Vec<T>>;

fn transpose(matrix: &Matrix<char>) -> Matrix<char> {
    if matrix.is_empty() {
        return vec![];
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut result = vec![vec!['.'; num_rows]; num_cols];
    for i in 0..num_rows {
        for j in 0..num_cols {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn main() {
    let input = include_str!("input.txt");
    let matrix: Matrix<char> = input.lines().map(|s| s.chars().collect()).collect();

    // part 1, push N
    let mut hashmap: HashMap<Matrix<char>, i64> = HashMap::new();

    // find cycle len
    let mut new_matrix = matrix;
    let mut weights: Vec<i64> = Vec::new();

    let mut iteration = 0;
    let (cycle_length, initial_length) = loop {
        if let Some(&x) = hashmap.get(&new_matrix) {
            break (weights.len() as i64 - x, x);
        } else {
            hashmap.insert(new_matrix.clone(), iteration);
        }

        // push weight
        new_matrix = transpose(&new_matrix);
        let mut sum = 0;
        for line in &new_matrix {
            let l = line.len();
            let s: usize = line
                .iter()
                .enumerate()
                .filter(|(_, s)| **s == 'O')
                .map(|(idx, _)| l - idx)
                .sum();

            sum += s;
        }
        weights.push(sum as i64);
        // new_matrix = transpose(&new_matrix);

        // push N
        // new_matrix = transpose(&new_matrix);
        push_left(&mut new_matrix);
        new_matrix = transpose(&new_matrix);

        // part 2  n => w => s => e
        // push W
        push_left(&mut new_matrix);
        // push S
        new_matrix = transpose(&new_matrix);
        push_right(&mut new_matrix);
        new_matrix = transpose(&new_matrix);
        // push E
        push_right(&mut new_matrix);
        // println!("iteration: {iteration} => {:?}", new_matrix);
        iteration += 1;
    };

    let total_cycle = 1_000_000_000;
    let relevant_index = (total_cycle - initial_length) % cycle_length;
    println!(
        "{cycle_length} {:?}",
        weights[(initial_length + relevant_index) as usize]
    );

    // part 1
    // let mut sum = 0;
    // for line in &new_matrix {
    //     let l = line.len();
    //     let s: usize = line
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, s)| **s == 'O')
    //         .map(|(idx, _)| l - idx)
    //         .sum();
    //
    //     sum += s;
    // }
    //
    // println!("{:?}", sum);
}

fn push_left(new_matrix: &mut Matrix<char>) {
    for line in new_matrix.iter_mut() {
        for i in 0..line.len() {
            let mut current_index = i;
            let char = line[i];
            if char != 'O' {
                continue;
            }

            // move the O forward
            while current_index > 0 {
                let prev_char = line[current_index - 1];
                if prev_char == 'O' || prev_char == '#' {
                    break;
                }

                // swap current and prev char
                line.swap(current_index, current_index - 1);
                current_index -= 1;
            }
        }
    }
}

fn push_right(new_matrix: &mut Matrix<char>) {
    for line in new_matrix.iter_mut() {
        for i in (0..line.len() - 1).rev() {
            let mut current_index = i;
            let char = line[i];
            if char != 'O' {
                continue;
            }

            // move the O backward
            while current_index < line.len() - 1 {
                let next_char = line[current_index + 1];
                if next_char == 'O' || next_char == '#' {
                    break;
                }

                // swap current and prev char
                line.swap(current_index, current_index + 1);
                current_index += 1;
            }
        }
    }
}
