use crate::Reflection::{Horizontal, Vertical};

type Matrix<T> = Vec<Vec<T>>;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Reflection {
    Vertical(i64),
    Horizontal(i64),
}

const THRESHOLD: i64 = 1; // part 1 is 0, part 2 is 1

fn check_horizontal(matrix: &Matrix<char>) -> Option<Reflection> {
    for i in 1..matrix.len() {
        // part 1
        let mut diff: i64 = 0;

        let mut top = i as i64 - 1;
        let mut bottom = i as i64;

        while top >= 0 && bottom < matrix.len().try_into().unwrap() {
            let top_row = matrix[top as usize].clone();
            let bottom_row = matrix[bottom as usize].clone();
            let diff_count = top_row.iter().zip(bottom_row).filter(|(l, r)| *l != r).count();
            diff += diff_count as i64;
            top -= 1;
            bottom += 1;
        }

        if diff == THRESHOLD {
            return Some(Horizontal(i as i64));
        }
    }

    None
}

fn check_vertical(matrix: &Matrix<char>) -> Option<Reflection> {
    for i in 1..matrix[0].len() {
        let mut diff: i64 = 0;
        let mut left = i as i64 - 1;
        let mut right = i as i64;

        while left >= 0 && right < matrix[0].len().try_into().unwrap() {
            let diff_count = matrix.iter().map(|s| (s[left as usize], s[right as usize])).filter(|(l, r)| l != r).count();
            diff += diff_count as i64;
            left -= 1;
            right += 1;
        }

        if diff == THRESHOLD {
            return Some(Vertical(i as i64));
        }
    }

    None
}

fn solve(s: &Matrix<char>) -> i64 {
    let x = check_horizontal(s);
    if let Some(Horizontal(ans)) = x {
        println!("horizontal {ans}");
        return ans * 100;
    }
    let y = check_vertical(s);
    if let Some(Vertical(ans)) = y {
        println!("vertical {ans}");
        return ans;
    }
    0
}

fn main() {
    let input = include_str!("input.txt");
    let matrix: Vec<Matrix<char>> = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.chars().collect()).collect())
        .collect();

    println!("{:?}", solve(&matrix[0]));
    let ans: i64 = matrix.iter().map(|s| solve(s)).sum();
    println!("{:?}", ans);
}

