use std::fs;

// Part 1
// fn main() {
//     let file = fs::read_to_string("input.txt").expect("Should have been able to read this file");
//     let mut sum = 0;
//     for line in file.lines() {
//         let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
//         let f = digits.first().unwrap().to_digit(10).unwrap();
//         let l = digits.last().unwrap().to_digit(10).unwrap();
//         let total = f * 10 + l;
//         sum += total;
//     }
//
//     println!("{sum}");
// }

// Part 2
fn main() {
    let file = fs::read_to_string("input.txt").expect("Should have been able to read this file");
    let mut sum = 0;
    let mapping = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // let mut result = String::from(file);
    // for (word, digit) in mapping {
    //     result = result.replace(word, digit)
    // }
    // for (word, digit) in mapping.iter() {
    //     result = result.replace(word, digit)
    // }
    //
    // let r = 1..4;
    // for i in r {
    //     println!("{}", i);
    // }
    // for i in r {
    //     println!("{}", i);
    // }

    // let _ = mapping.to_vec();
    // let (val, val2) = mapping.get(1).unwrap();
    // println!("{val}");

    for line in file.lines() {
        let mut numbers: Vec<(i32, i32)> = Vec::new();
        for (index, character) in line.char_indices() {
            if character.is_digit(10) {
                numbers.push((index as i32, character.to_digit(10).unwrap() as i32))
            }
        }
        for (word, digit) in mapping {
            for (i, _) in line.match_indices(word) {
                // somehow the borrow checker said use of moved value, but not correct here... since its copied
                numbers.push((i as i32, digit));
            }
        }
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?}", numbers);
        // huh borrow checker mention cannot move, this is also wrong?
        let f = numbers.first().unwrap().1;
        let l = numbers.last().unwrap().1;
        let total = f * 10 + l;
        sum += total;
    }

    println!("{sum}");
}
//
// fn main() {
//     let input = "eightwo";
//     let search_str = "eight";
//
//     for (i, _) in input.match_indices(search_str) {
//         println!("Found {search_str} at index {i}")
//     }
// }
// fn main() {
//     let mut value = 10; // This is a mutable i32 variable
//
//     // Function that takes a mutable reference to an i32
//     fn mutate_value(v: &mut i32) {
//         *v += 5; // Mutating the value by adding 5 to it
//     }
//
//     println!("Original value: {}", value);
//
//     // Creating a mutable reference to the value
//     mutate_value(&mut value);
//
//     println!("Mutated value: {}", value);
// }
