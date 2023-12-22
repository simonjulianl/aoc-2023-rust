use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Block {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}


fn main() {
    let input = include_str!("input.txt");

    let mut all_blocks = Vec::new();
    for line in input.lines() {
        let (from, to) = line.split_once('~').unwrap();
        let f_vec: Vec<i32> = from.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let t_vec: Vec<i32> = to.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let new_block = Block {
            x: (f_vec[0], t_vec[0]),
            y: (f_vec[1], t_vec[1]),
            z: (f_vec[2], t_vec[2]),
        };
        all_blocks.push(new_block);
    }

    all_blocks.sort_by(|a, b| a.z.0.cmp(&b.z.0)); // sort by the z index

    // create a map (x, y) where each grid contains (z, idx)
    let n = all_blocks.len();
    let mut highest_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut bad_blocks = HashSet::new();

    for (idx, block) in all_blocks.iter().enumerate() {
        let mut max_height = -1;
        let mut support_set = HashSet::new();

        for x in block.x.0..=block.x.1 {
            for y in block.y.0..=block.y.1 {
                let (curr_height, block_idx) = highest_map.entry((x, y)).or_insert((0, -1));
                if *curr_height + 1 > max_height {
                    max_height = *curr_height + 1;

                    // resert the support set
                    support_set.clear();
                    support_set.insert(*block_idx);
                } else if *curr_height + 1 == max_height {
                    support_set.insert(*block_idx);
                }
            }
        }

        for item in &support_set {
            if *item != -1 {
                graph[*item as usize].push(idx);
            }
        }

        if support_set.len() == 1 {
            // only supported by 1 block, then the supporting block is GG
            bad_blocks.insert(support_set.iter().next().cloned().unwrap());
        }

        let fall_height = block.z.0 - max_height;
        let new_height = block.z.1 - fall_height;
        for x in block.x.0..=block.x.1 {
            for y in block.y.0..=block.y.1 {
                highest_map.insert((x, y), (new_height, idx as i32));
            }
        }
    }

    // println!("{:?}", all_blocks);
    println!("{:?}", graph);

    // remove the ground
    bad_blocks.remove(&-1);
    println!("{:?}", n - bad_blocks.len());


    // part 2
    // count indegree, tbh similar to topo sort
    let mut indegree = vec![0; n];
    for neighbours in &graph {
        for &item in neighbours {
            indegree[item] += 1;
        }
    }

    let mut total_ans = 0;
    for &item in &bad_blocks {
        let mut new_indegree = indegree.clone();
        let mut q = VecDeque::new();
        let mut ans = 0; // other bricks
        q.push_back(item);

        while let Some(x) = q.pop_front() {
            for &n in &graph[x as usize] {
                new_indegree[n] -= 1;
                if new_indegree[n] == 0 {
                    q.push_back(n as i32);
                    ans += 1;
                }
            }
        }

        total_ans += ans;
    }

    println!("{total_ans}");
}