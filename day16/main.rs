use std::cmp::max;
use std::collections::{HashSet, VecDeque};

type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Tile {
    #[default]
    Empty,
    HSplitter,
    VSplitter,
    Upmirror,
    Downmirror,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'-' => Self::HSplitter,
            b'|' => Self::VSplitter,
            b'/' => Self::Upmirror,
            b'\\' => Self::Downmirror,
            _ => panic!("oh no"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Ray {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Ray {
    fn reflect(&mut self, map: &Matrix<Tile>) -> Vec<Ray> {
        let current_tile = map[self.row][self.col];
        match current_tile {
            Tile::Empty => {}
            Tile::Upmirror => {
                match self.direction {
                    Direction::East => self.direction = Direction::North,
                    Direction::North => self.direction = Direction::East,
                    Direction::West => self.direction = Direction::South,
                    Direction::South => self.direction = Direction::West,
                }
            }
            Tile::Downmirror => {
                match self.direction {
                    Direction::East => self.direction = Direction::South,
                    Direction::South => self.direction = Direction::East,
                    Direction::West => self.direction = Direction::North,
                    Direction::North => self.direction = Direction::West,
                }
            }
            Tile::HSplitter => {
                match self.direction {
                    Direction::East | Direction::West => {}
                    Direction::South | Direction::North => {
                        let east_ray = Ray {
                            row: self.row,
                            col: self.col,
                            direction: Direction::East,
                        };

                        let west_ray = Ray {
                            row: self.row,
                            col: self.col,
                            direction: Direction::West,
                        };

                        return vec![east_ray, west_ray];
                    }
                }
            }
            Tile::VSplitter => {
                match self.direction {
                    Direction::South | Direction::North => {}
                    Direction::West | Direction::East => {
                        let south_ray = Ray {
                            row: self.row,
                            col: self.col,
                            direction: Direction::South,
                        };

                        let north_ray = Ray {
                            row: self.row,
                            col: self.col,
                            direction: Direction::North,
                        };

                        return vec![south_ray, north_ray];
                    }
                }
            }
        }

        vec![self.clone()]
    }
}

fn parse(line: &str) -> Vec<Tile> {
    line.bytes().map(Tile::from).collect()
}

fn main() {
    let input = include_str!("input.txt");
    let map: Matrix<Tile> = input.lines().map(parse).collect();
    let row_limit = map.len() as i64;
    let col_limit = map[0].len() as i64;

    let mut overall_ans = 0;
    for i in 0..row_limit {
        // start from the left side
        let mut initial_ray = Ray {
            row: i as usize,
            col: 0,
            direction: Direction::East,
        };
        let mut ans = get_ans(&map, row_limit, col_limit, initial_ray);
        overall_ans = max(ans, overall_ans);

        // start from the right side
        initial_ray = Ray {
            row: i as usize,
            col: (col_limit - 1) as usize,
            direction: Direction::West,
        };
        ans = get_ans(&map, row_limit, col_limit, initial_ray);
        overall_ans = max(ans, overall_ans);
        // println!("{:?}", ans);
    }

    for i in 0..col_limit {
        // start from the top side
        let mut initial_ray = Ray {
            row: 0,
            col: i as usize,
            direction: Direction::South,
        };
        let mut ans = get_ans(&map, row_limit, col_limit, initial_ray);
        overall_ans = max(ans, overall_ans);

        // start from the bottom side
        initial_ray = Ray {
            row: (row_limit - 1) as usize,
            col: i as usize,
            direction: Direction::North,
        };
        ans = get_ans(&map, row_limit, col_limit, initial_ray);
        overall_ans = max(ans, overall_ans);
    }

    println!("{:?}", overall_ans);
}

fn get_ans(map: &Matrix<Tile>, row_limit: i64, col_limit: i64, initial_ray: Ray) -> i64 {
    let mut queue: VecDeque<Ray> = VecDeque::new();
    queue.push_back(initial_ray);

    let mut visited_map = vec![vec![false; col_limit as usize]; row_limit as usize];
    let mut seen_rays: HashSet<Ray> = HashSet::new();
    while !queue.is_empty() {
        if let Some(mut r) = queue.pop_front() {
            visited_map[r.row][r.col] = true;
            seen_rays.insert(r);
            let reflected_rays = r.reflect(&map);
            for item in reflected_rays {
                let dir = item.direction;
                let new_row = match item.direction {
                    Direction::East => item.row as i64,
                    Direction::South => item.row as i64 + 1,
                    Direction::West => item.row as i64,
                    Direction::North => item.row as i64 - 1,
                };
                let new_col = match item.direction {
                    Direction::East => item.col as i64 + 1,
                    Direction::South => item.col as i64,
                    Direction::West => item.col as i64 - 1,
                    Direction::North => item.col as i64,
                };
                if 0 <= new_row && new_row < row_limit && 0 <= new_col && new_col < col_limit {
                    let new_ray = Ray {
                        row: new_row as usize,
                        col: new_col as usize,
                        direction: dir,
                    };
                    if !seen_rays.contains(&new_ray) {
                        queue.push_back(new_ray);
                    }
                }
            }
        }
    }
    let ans: i64 = visited_map
        .iter()
        .fold(0, |acc, x| acc + x.iter().fold(0, |acc, &x| acc + if x { 1 } else { 0 }));
    ans
}
