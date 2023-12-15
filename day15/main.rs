const MODULO: i64 = 256;
const MULTIPLIER: i64 = 17;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Lens<'a> {
    label: &'a str,
    focal_length: i64,
}

fn main() {
    let input = include_str!("input.txt");
    // part 1
    // let ans: i64 = input.split(',').map(|s| hash(s)).sum();

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    for s in input.split(',') {
        if let Some((label, _)) = s.split_once('-') {
            // remove operation
            let b = hash(label) as usize;
            let relevant_chain = &mut boxes[b];
            let idxs: Vec<usize> = relevant_chain
                .iter()
                .enumerate()
                .filter(|(_s, &b)| b.label == label)
                .map(|(s, _)| s)
                .collect();

            // remove relevant idxs
            if !idxs.is_empty() {
                relevant_chain.remove(*idxs.first().unwrap());
            }
            // println!("{:?}", idxs);
        } else if let Some((label, focal)) = s.split_once('=') {
            let b = hash(label) as usize;
            let relevant_chain = &mut boxes[b];
            let idxs: Vec<usize> = relevant_chain
                .iter()
                .enumerate()
                .filter(|(_s, &b)| b.label == label)
                .map(|(s, _)| s)
                .collect();
            if idxs.is_empty() {
                // insert at the back
                let new_lens = Lens {
                    label,
                    focal_length: focal.parse::<i64>().unwrap_or_default(),
                };
                relevant_chain.push(new_lens);
            } else {
                // replace
                relevant_chain[*idxs.first().unwrap()].focal_length =
                    focal.parse::<i64>().unwrap_or_default();
            }
        }
    }
    let ans: i64 = boxes.iter().enumerate().fold(0, |acc, (box_num, x)| {
        acc + x.iter().enumerate().fold(0, |acc, (idx, &s)| {
            acc + s.focal_length * (box_num + 1) as i64 * (idx + 1) as i64
        })
    });
    println!("{:?}", ans);
}

fn hash(s: &str) -> i64 {
    s.chars()
        .map(|s| s as i64)
        .fold(0, |acc, x| ((acc + x) * MULTIPLIER) % MODULO)
}
