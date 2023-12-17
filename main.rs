use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

type Matrix<T> = Vec<Vec<T>>;


fn parse(line: &str) -> Vec<usize> {
    line.chars().map(|s| s.to_digit(10).unwrap_or_default() as usize).collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn add(&self, row: usize, col: usize, row_max: usize, col_max: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North if row >= 1 => Some((row - 1, col)),
            Direction::South if row < row_max - 1 => Some((row + 1, col)),
            Direction::East if col < col_max - 1 => Some((row, col + 1)),
            Direction::West if col >= 1 => Some((row, col - 1)),
            _ => None
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Node {
    row: usize,
    col: usize,
    straight_counter: usize,
    direction: Direction,
}

// impl Hash for Node {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.row.hash(state);
//         self.col.hash(state);
//         self.straight_counter.hash(state);
//         self.direction.hash(state);
//     }
// }

// impl PartialOrd<Self> for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other)) // call the cmp
//     }
// }
//
// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.dist.cmp(&self.dist) // create min heap
//     }
// }


fn main() {
    let input = include_str!("input.txt");
    let map: Matrix<usize> = input.lines().map(parse).collect();

    let max_col = map[0].len();
    let max_row = map.len();

    let mut distance: HashMap<Node, usize> = HashMap::new();
    let mut pq: BinaryHeap<(i64, Node)> = BinaryHeap::new();

    let starts = [
        Node {
            row: 0,
            col: 0,
            straight_counter: 1,
            direction: Direction::South,
        },
        Node {
            row: 0,
            col: 0,
            straight_counter: 1,
            direction: Direction::East,
        },
    ];

    // let test1 =
    //  Node {
    //         row: 0,
    //         col: 0,
    //         dist: 0,
    //         straight_counter: 1,
    //         direction: Direction::East,
    //     };
    // let test2 =
    //  Node {
    //         row: 0,
    //         col: 0,
    //         dist: 2,
    //         straight_counter: 1,
    //         direction: Direction::East,
    //     };
    //
    // distance.insert(test1, 0);
    // println!("{:?}", distance.contains_key(&test2));
    for start in starts {
        distance.insert(start, 0);
        pq.push((0, start));
    }

    while let Some(
        (cost, Node {
            row,
            col,
            straight_counter,
            direction,
        })
    ) = pq.pop() {
        // for part 2, you need to check that the direction is enough to stop
        if row == (max_row - 1) && col == (max_col - 1) && straight_counter >= 4 {
            println!("{cost}");
            break;
        }
        let mut push = |d: Direction, s: usize| {
            let n = d.add(row, col, max_row, max_col);
            if let Some((new_row, new_col)) = n {
                let next_cost = (-cost) as usize + map[new_row][new_col];
                let new_node = Node {
                    row: new_row,
                    col: new_col,
                    straight_counter: s,
                    direction: d,
                };

                if !distance.contains_key(&new_node) || next_cost < *distance.get(&new_node).unwrap() {
                    let negated_cost = -(next_cost as i64);
                    pq.push((negated_cost, new_node));
                    distance.insert(new_node, next_cost as usize);
                }
            }
        };

        if straight_counter < 10 { // part 1 is 3
            // try to push straight
            push(direction, straight_counter + 1);
        }

        if straight_counter >= 4 { // literally just change this for part 1 and 2
            // try to turn
            push(direction.left(), 1);
            push(direction.right(), 1);
        }
    }

    // println!("{:?}", distance);
}

