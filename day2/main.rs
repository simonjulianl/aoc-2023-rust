use std::fs;
use std::cmp::max;

// Part 1
// fn main() {
//     fn check_color(color: &str, value: i32) -> bool {
//         match color {
//             "blue" => value <= 14,
//             "green" => value <= 13,
//             "red" => value <= 12,
//             _ => false,
//         }
//     }
//
//     let file = fs::read_to_string("input.txt").expect("Should have been able to parse");
//     let mut sum = 0;
//     for l in file.lines() {
//         let parts: Vec<&str> = l.split(':').collect();
//         let numeric_part: String = parts.first().unwrap()
//             .chars()
//             .skip_while(|c| !c.is_digit(10))
//             .collect();
//         let game_id = numeric_part.parse::<i32>().unwrap();
//
//         // 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//         let games: Vec<&str> = parts.get(1).unwrap().split(';').collect();
//         let mut is_possible = true;
//         for item in &games {
//             let items: Vec<&str> = item.trim_start().split(", ").collect();
//             for x in &items {
//                 let pair: Vec<&str> = x.split(' ').collect();
//                 let value = pair.first().unwrap();
//                 let key = pair.last().unwrap();
//                 if !check_color(key, value.parse::<i32>().unwrap()) {
//                     is_possible = false;
//                 }
//             }
//         }
//
//         if is_possible {
//             sum += game_id;
//         }
//     }
//     println!("{:?}", sum);
// }

// part 2
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should have been able to parse");
    let mut sum = 0;
    for l in file.lines() {
        let parts: Vec<&str> = l.split(':').collect();

        // 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        let games: Vec<&str> = parts.get(1).unwrap().split(';').collect();
        let mut new_game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for item in &games {
            let items: Vec<&str> = item.trim_start().split(", ").collect();
            for x in &items {
                let pair: Vec<&str> = x.split(' ').collect();
                let value = pair.first().unwrap().parse::<i32>().unwrap();
                let key = pair.last().unwrap();
                match key {
                    &"blue" => {
                        let max_number = max(value, new_game.blue);
                        new_game.blue = max_number;
                    }
                    &"green" => {
                        let max_number = max(value, new_game.green);
                        new_game.green = max_number;
                    }
                    &"red" => {
                        let max_number = max(value, new_game.red);
                        new_game.red = max_number;
                    }
                    _ => continue,
                }
            }
        }

        let power = new_game.blue * new_game.green * new_game.red;
        sum += power;
    }
    println!("{:?}", sum);
}
