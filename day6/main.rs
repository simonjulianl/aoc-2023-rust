use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");

    let file_array: Vec<&str> = file.lines().collect();
    let times: Vec<&str> = file_array.first().unwrap().split(':').collect();
    let time_int = parse_int(times);

    let distances: Vec<&str> = file_array.last().unwrap().split(':').collect();
    let distance_int = parse_int(distances);

    let ans = find_ways(time_int, distance_int);
    // for i in 0..distance_int.len() {
    //     ans *= find_ways(time_int[i], distance_int[i]);
    // }

    println!("{:?}", ans);
}

// // part 1
// fn parse_int(arr: Vec<&str>) -> Vec<i64> {
//     arr[1].split(' ')
//         .into_iter()
//         .map(|s| s.parse().unwrap_or_default())
//         .filter(|s| *s > 0)
//         .collect()
// }

// part 2
fn parse_int(arr: Vec<&str>) -> i64 {
    arr[1].replace(' ', "")
        .parse::<i64>().unwrap_or_default()
}

fn find_ways(total_time: i64, distance: i64) -> i64 {
    let mut sum = 0;
    for i in 1..total_time {
        // 1 and total time is not included since its 0
        let d = calculate_distance(i, total_time);
        if d > distance {
            sum += 1
        }
    }

    sum
}

fn calculate_distance(hold_time: i64, total_time: i64) -> i64 {
    let speed = hold_time;
    let distance = speed * (total_time - hold_time);
    distance
}