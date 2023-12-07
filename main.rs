use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum CardType {
    FIVE,
    FOUR,
    FH,
    TRIPS,
    TWOP,
    OP,
    HC,
}

fn get_card_type(cards: &str) -> CardType {
    let mut char_count = HashMap::new();

    for ch in cards.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    let mut five_count = 0;
    let mut four_count = 0;
    let mut trips_count = 0;
    let mut pair_count = 0;
    let mut single_count = 0;
    let mut joker_count = 0;
    for (j, count) in &char_count {
        if *j == 'J' {
            joker_count = *count;
            continue;
        }

        match count {
            5 => five_count += 1,
            4 => four_count += 1,
            3 => trips_count += 1,
            2 => pair_count += 1,
            1 => single_count += 1,
            _ => panic!("Oh no, what type of card is this"),
        }
    }

    return if joker_count == 0 {
        if five_count == 1 {
            CardType::FIVE
        } else if four_count == 1 && single_count == 1 {
            CardType::FOUR
        } else if trips_count == 1 && pair_count == 1 {
            CardType::FH
        } else if trips_count == 1 && single_count == 2 {
            CardType::TRIPS
        } else if pair_count == 2 && single_count == 1 {
            CardType::TWOP
        } else if pair_count == 1 && single_count == 3 {
            CardType::OP
        } else {
            CardType::HC
        }
    } else if joker_count == 1 {
        if four_count == 1 {
            CardType::FIVE // 0 single
        } else if trips_count == 1 {
            assert_eq!(single_count, 1);
            CardType::FOUR // 1 single
        } else if pair_count == 2 {
            assert_eq!(single_count, 0);
            CardType::FH
        } else if pair_count == 1 {
            // single 2 pair 1
            assert_eq!(single_count, 2);
            CardType::TRIPS
        } else {
            // 4 single
            assert_eq!(single_count, 4);
            CardType::OP
        }
    } else if joker_count == 2 {
        if trips_count == 1 {
            assert_eq!(single_count, 0);
            CardType::FIVE
        } else if pair_count == 1 {
            assert_eq!(single_count, 1);
            CardType::FOUR
        } else {
            // 2 single => 3 single
            assert_eq!(single_count, 3);
            CardType::TRIPS
        }
    } else if joker_count == 3 {
        if pair_count == 1 {
            CardType::FIVE
        } else {
            // 2 single
            assert_eq!(single_count, 2);
            CardType::FOUR
        }
    } else if joker_count == 4 {
        CardType::FIVE
    } else { // 5 joker
        CardType::FIVE
    }
}

fn convert_char_to_int(ch: &char) -> i64 {
    // part 1
    // let custom_order = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    // part 2 the jack is gone
    let custom_order = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    custom_order.iter().position(|c| c == ch).unwrap() as i64
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");
    let mut data: Vec<(&str, CardType, i64)> = Vec::new();
    for l in file.lines() {
        let card_bid: Vec<&str> = l.split(' ').collect();
        let bid = card_bid.last().unwrap().parse::<i64>().unwrap();
        let card = card_bid.first().unwrap();
        let t = get_card_type(card);
        if card.contains('J') {
            println!("{:?} {:?}", card, t);
        }
        data.push((card, t, bid));
    }

    // let test = CardType::TWOP < CardType::FH;
    data.sort_by(|(c1, ty1, _), (c2, ty2, _)| {
        return if ty1 == ty2 {
            for (idx, ch) in c1.char_indices() {
                // println!("{:?} {c2}", c1);
                let char2 = c2.chars().nth(idx).unwrap();
                let first_value = convert_char_to_int(&ch);
                let second_value = convert_char_to_int(&char2);
                let result = second_value.cmp(&first_value);
                // println!("{:?} {c2} {:?} {first_value} {second_value}", c1, result);
                if result != Ordering::Equal {
                    return result;
                }
            }
            Ordering::Equal
        } else {
            ty2.cmp(ty1)
        };
    });

    let mut result = 0;
    for (idx, (_, _, bid)) in data.iter().enumerate() {
        result += ((idx + 1) as i64) * bid;
    }
    // println!("{:?}", data);
    println!("{:?}", result);
}

// fn main() {
//     let mut value = 42;
//     let reference = &&value;
//
//     // Dereference the double reference to obtain a single reference
//     let single_reference = *reference;
//
//     // Clone the underlying value of the single reference
//     let cloned_value = reference.clone();
//
//     value += 5;
//     println!("Original value: {}", value);
//     println!("Cloned value: {}", cloned_value);
// }