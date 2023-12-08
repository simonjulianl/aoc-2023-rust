use std::collections::HashMap;
use std::fs;
use std::iter::Cycle;
use std::str::Chars;

use num::integer::lcm;

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let contents: Vec<&str> = file.lines().collect();
    let direction = contents.first().unwrap();

    if let Some(s) = contents.get(2..) {
        for l in s {
            let key_value: Vec<&str> = l.split('=').collect();
            let key = key_value.first().unwrap().trim();
            let value: Vec<&str> = key_value
                .last()
                .unwrap()
                .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                .split(',')
                .map(|s| s.trim())
                .collect();

            let tuple = (value[0], value[1]);
            map.insert(key, tuple);
        }
    }

    let mut ans = 0;
    let mut infinite_cycle = direction.chars().cycle();

    // part 1
    // let mut current = "AAA";
    // let destination = "ZZZ";

    // part 2
    let current: Vec<&str> = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect();

    // println!("{:?}", current);

    let ans: i64 = current
        .iter()
        .map(|s| get_single_ans(&mut map, &mut infinite_cycle, s))
        .fold(1, |acc, num| lcm(acc, num));
    println!("{:?}", ans);


    // attempt 1 brute force
    // let answer = loop {
    //     ans += 1;
    //     let current_char = infinite_cycle.next().unwrap();
    //     current = current
    //         .iter()
    //         .map(|s| get_next(s, &current_char))
    //         .collect();
    //     // println!("{:?}", current);
    //
    //     if current.iter().all(|c| c.ends_with('Z')) {
    //         break ans;
    //     }
    // };

    // println!("{answer}");
}

fn get_single_ans(
    map: &mut HashMap<&str, (&str, &str)>,
    infinite_cycle: &mut Cycle<Chars>,
    current: &str,
) -> i64 {
    let mut ans = 0;
    let mut curr = current;

    let get_next = |dir: &char, curr: &str| {
        let options = map.get(curr).unwrap();
        match dir {
            'L' => options.0,
            'R' => options.1,
            _ => panic!("Cannot be here"),
        }
    };

    loop {
        ans += 1;
        let current_char = infinite_cycle.next().unwrap();
        curr = get_next(&current_char, &curr);

        if curr.ends_with('Z') {
            break ans;
        }
    }
}