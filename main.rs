use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should have been able to read this file");
    let mut sum = 0;
    for line in file.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let f = digits.first().unwrap().to_digit(10).unwrap();
        let l = digits.last().unwrap().to_digit(10).unwrap();
        let total = f * 10 + l;
        sum += total;
    }

    println!("{sum}");
}

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
