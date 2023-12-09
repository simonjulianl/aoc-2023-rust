use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("can parse");
    let mut s = 0;
    for line in file.lines() {
        let nums: Vec<i64> = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
        let mut last_nums: Vec<i64> = vec![*nums.first().unwrap()];
        // part 1
        // let mut last_nums: Vec<i64> = vec![*nums.last().unwrap()];
        let mut curr = nums.clone();
        let mut differences: Vec<i64>;
        loop {
            differences = curr.windows(2).map(|pair| pair[1] - pair[0]).collect();

            // part 1
            // let last_number = *differences.last().unwrap();
            let first_number = *differences.first().unwrap();
            // last_nums.push(last_number);
            last_nums.push(first_number);
            if differences.iter().all(|&s| s == differences[0]) {
                break;
            } else {
                curr = differences;
            }
        };

        // let result = last_nums.iter().rfold(0, |acc, &x| acc + x);
        // part 2
        let result = last_nums.iter().rfold(0, |acc, &x| x - acc);
        // println!("{:?}", last_nums);
        // println!("{:?}", result);
        s += result;
    }
    println!("{:?}", s);
}
