use std::fs;

// part 1
// fn translate_mapping(file_array: &Vec<&str>, seeds: &mut Vec<i64>, idx: usize) {
//     let mut is_modified = vec![false; seeds.len()];
//     let mut i = idx + 1;
//     let mut current_line = file_array[i];
//     while current_line != "" {
//         i += 1;
//         // println!("{:?}", current_line);
//         let value: Vec<&str> = current_line.split(' ').collect();
//         let value_int: Vec<i64> = value.iter().map(|s| s.parse().unwrap_or_default()).collect();
//         let source = value_int[1];
//         let destination = value_int[0];
//         let range = value_int[2];
//
//         // translate each seed
//         for (ix, s) in seeds.clone().iter().enumerate() {
//             if is_modified[ix] {
//                 continue;
//             }
//
//             if source <= *s && *s <= source + range {
//                 let difference = *s - source;
//                 // println!("{destination} {difference}");
//                 let result = destination + difference;
//                 is_modified[ix] = true;
//                 seeds[ix] = result;
//             }
//         }
//
//
//         if i == file_array.len() {
//             break;
//         }
//         current_line = file_array[i];
//     }
// }
//
// fn main() {
//     let file = fs::read_to_string("input.txt").expect("can parse");
//
//     let mut seeds: Vec<i64> = Vec::new();
//
//     let file_array: Vec<&str> = file.lines().collect();
//
//     for (idx, line) in file.lines().enumerate() {
//         if line.starts_with("seeds") {
//             let parsed_line: Vec<&str> = line.split(':').collect();
//             let result: Vec<&str> = parsed_line.last().unwrap().trim().split(' ').collect();
//             let result_int: Vec<i64> = result.iter().map(|s| s.parse().unwrap_or_default()).collect();
//             for i in (0..result_int.len()).step_by(2) {
//                 let r = result_int[i + 1];
//                 let base = result_int[i];
//                 for k in 0..r {
//                     seeds.push(base + k);
//                 }
//             }
//         } else if line.starts_with("seed-to-soil") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("soil-to-fertilizer") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("fertilizer-to-water") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("water-to-light") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("light-to-temperature") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("temperature-to-humidity") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         } else if line.starts_with("humidity-to-location") {
//             translate_mapping(&file_array, &mut seeds, idx);
//         }
//     }
//
//     println!("{:?}", seeds.iter().min().unwrap());
// }

// use std::collections::HashMap;
//
// // Function to try modifying a HashMap without using &mut
// fn try_modify_map(map: HashMap<String, i32>) {
//     // Insert or update entries in the hashmap
//     // This will attempt to modify the copied hashmap, not the original one
//     map.insert(String::from("Key1"), 10);
//     map.insert(String::from("Key2"), 20);
//     map.insert(String::from("Key3"), 30);
// }
//
// fn main() {
//     let mut my_map: HashMap<String, i32> = HashMap::new();
//
//     // Insert initial entries into the hashmap
//     my_map.insert(String::from("InitialKey"), 100);
//
//     // Call the function and pass the hashmap directly without &mut
//     try_modify_map(my_map);
//
//     // Display the original hashmap to see if it was modified
//     for (key, value) in &my_map {
//         println!("Key: {}, Value: {}", key, value);
//     }
// }

fn translate_mapping(file_array: &Vec<&str>, seeds: &mut Vec<(i64, i64)>, idx: usize) {
    let mut new_array: Vec<(i64, i64)> = Vec::new();
    let mut i = idx + 1;
    let mut current_line = file_array[i];
    while current_line != "" {
        i += 1;

        let value: Vec<&str> = current_line.split(' ').collect();
        let value_int: Vec<i64> = value.iter().map(|s| s.parse().unwrap_or_default()).collect();

        let range = value_int[2];

        let source = value_int[1];
        let source_end = source + range - 1;

        let destination = value_int[0];
        let destination_end = destination + range - 1;

        // println!("{:?}", seeds);
        // translate each seed
        for (start, end) in seeds.clone() {
            // println!("seed: {start} {end}");
            seeds.remove(0);
            // for part 2, need to check range by range, replace all [source, source + range) to [destination, destination + range)

            // check if the translation intersect with range
            // case 1, the entire start - end is within translation range
            if source <= start && end <= source_end {
                let difference = start - source;
                let difference_end = end - source;
                let result = destination + difference;
                let result_end = destination + difference_end;
                new_array.push((result, result_end));
                println!("case 1, total overlapping ");
            } else if start < source && source <= end && end <= source_end {
                // case 2, start <= source but overlap, end <= source_end
                let difference_end = end - source;
                let result_end = destination + difference_end;
                new_array.push((destination, result_end));
                let new_pair = (start, source - 1);
                seeds.push(new_pair);
                println!("case 2, left overlapping");
            } else if source <= start && start <= source_end && end > source_end {
                // case 3, start >= source but overlap, end >= source_end
                let difference = start - source;
                let result = destination + difference;
                new_array.push((result, destination_end));
                let new_pair = (source_end + 1, end);
                seeds.push(new_pair);
                println!("case 3, right overlapping");
            } else if start < source && end > source_end {
                // case 4 the start and end envelopes the range, splits to 3 range
                let first_range = (start, source - 1);
                let third_range = (source_end + 1, end);

                new_array.push((destination, destination_end));
                seeds.push(first_range);
                seeds.push(third_range);
                println!("case 4, overlapping overarching");
            } else {
                // no overlapping
                let new_pair = (start, end);
                println!("no overlapping {:?}", new_pair);
                seeds.push(new_pair);
            }
        }

        println!("{source}-{source_end} => {destination}-{destination_end} {:?} sum: {:?}", seeds, sum_vec(&seeds));

        if i == file_array.len() {
            break;
        }
        current_line = file_array[i];
    }

    for item in &new_array {
        seeds.push(*item);
    }
}

fn sum_vec(v: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for item in v {
        let r = item.1 - item.0 + 1;
        sum += r;
    }
    return sum;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");

    let mut seeds: Vec<(i64, i64)> = Vec::new();

    let file_array: Vec<&str> = file.lines().collect();

    for (idx, line) in file.lines().enumerate() {
        if line.starts_with("seeds") {
            let parsed_line: Vec<&str> = line.split(':').collect();
            let result: Vec<&str> = parsed_line.last().unwrap().trim().split(' ').collect();
            let result_int: Vec<i64> = result.iter().map(|s| s.parse().unwrap_or_default()).collect();
            for i in (0..result_int.len()).step_by(2) {
                let base = result_int[i];
                let r = result_int[i + 1];
                seeds.push((base, base + r - 1));
            }
        } else if line.starts_with("seed-to-soil") {
            println!("Phase 1");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("soil-to-fertilizer") {
            println!("Phase 2");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("fertilizer-to-water") {
            println!("Phase 3");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("water-to-light") {
            println!("Phase 4");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("light-to-temperature") {
            println!("Phase 5");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("temperature-to-humidity") {
            println!("Phase 6");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        } else if line.starts_with("humidity-to-location") {
            println!("Phase 7");
            println!("{:?} {:?}", seeds, sum_vec(&seeds));
            translate_mapping(&file_array, &mut seeds, idx);
        }
    }

    println!("{:?}", seeds);
    println!("{:?}", seeds.iter().min_by_key(|&k| k.0).unwrap().0);
}
