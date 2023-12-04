use std::fs;

// part 1
// fn main() {
//     let file = fs::read_to_string("input.txt").expect("can parse");
//
//     let mut score = 0;
//
//     for line in file.lines() {
//         let result: Vec<&str> = line.split('|').collect();
//         let first_part = result.first().unwrap();
//         let first_part_parsed: Vec<&str> = first_part.split(':').collect();
//         let winning_numbers: Vec<&str> = first_part_parsed.last().unwrap().trim().split(' ').collect();
//         let filtered_winning_numbers: Vec<&str> = winning_numbers.iter().filter(|&&x| x != "").cloned().collect();
//
//         let my_card: Vec<&str> = result.last().unwrap().trim().split(' ').collect();
//         let filtered_my_cards: Vec<&str> = my_card.iter().filter(|&&x| x != "").cloned().collect();
//
//         let mut count = 0;
//         for element in &filtered_my_cards {
//             if filtered_winning_numbers.contains(element) {
//                 count += 1;
//             }
//         }
//
//         if count > 0 {
//             let cur_score = i32::pow(2, count - 1);
//             score += cur_score;
//         }
//     }
//
//     println!("{:?}", score);
// }

// part 2
fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");
    let mut card_counter = vec![1;file.lines().count()];

    for (idx, line)in file.lines().enumerate() {
        let result: Vec<&str> = line.split('|').collect();
        let first_part = result.first().unwrap();
        let first_part_parsed: Vec<&str> = first_part.split(':').collect();
        let winning_numbers: Vec<&str> = first_part_parsed.last().unwrap().trim().split(' ').collect();
        let filtered_winning_numbers: Vec<&str> = winning_numbers.iter().filter(|&&x| x != "").cloned().collect();

        let my_card: Vec<&str> = result.last().unwrap().trim().split(' ').collect();
        let filtered_my_cards: Vec<&str> = my_card.iter().filter(|&&x| x != "").cloned().collect();

        let mut count = 0;
        for element in &filtered_my_cards {
            if filtered_winning_numbers.contains(element) {
                count += 1;
            }
        }

        let multiplier = card_counter[idx];
        while count > 0 {
            card_counter[idx + count] += multiplier;
            count -= 1;
        }
    }

    let s: i32 = card_counter.iter().sum();
    println!("{:?}", s);
}
