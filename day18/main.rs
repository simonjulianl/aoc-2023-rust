use num::abs;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" | "0" => Self::Right,
            "D" | "1" => Self::Down,
            "L" | "2" => Self::Left,
            "U" | "3" => Self::Up,
            _ => panic!("oh no"),
        }
    }
}

impl Direction {
    fn advance(&self, (row, col): (i64, i64), amount: i64) -> (i64, i64) {
        match self {
            Direction::Right => (row, col + amount),
            Direction::Down => (row + amount, col),
            Direction::Up => (row - amount, col),
            Direction::Left => (row, col - amount),
        }
    }
}

#[derive(Debug)]
struct Line {
    dir: Direction,
    amount: i64,
    // color: String,
}

fn parse(line: &str) -> Line {
    let l: Vec<&str> = line.split(' ').collect();
    Line {
        dir: Direction::from(&l[2][7..8]),
        // part 2
        amount: i64::from_str_radix(&l[2][2..7], 16).unwrap(),
        // part 1
        // amount: l[1].parse::<i64>().unwrap(),
        // color: l[2].to_string(),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let parsed_inputs: Vec<Line> = input.lines().map(parse).collect();

    let points = create_points(&parsed_inputs);
    let (boundary_points, area) = get_area_boundary_points(&points);

    // picks theorem, A = i + b/2 - 1 hence i = A - b/2 + 1
    let interior_point = area - boundary_points / 2 + 1;

    println!("{:?}", boundary_points + interior_point);
}

fn create_points(parsed_inputs: &Vec<Line>) -> Vec<(i64, i64)> {
    let mut current_point = (0, 0);
    let mut points = vec![current_point];
    for line in parsed_inputs {
        let new_point = line.dir.advance(current_point, line.amount);
        points.push(new_point);
        current_point = new_point;
    }
    points
}

fn get_area_boundary_points(points: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut boundary_points = 0;
    let mut area = 0;
    for i in 0..points.len() {
        let (a1, b1) = points[i];
        let (a2, b2) = points[(i + 1) % points.len()];
        // This is actually sticking the puzzle like lego
        boundary_points += abs(a1 - a2) + abs(b1 - b2);
        area += (a1 * b2) - (a2 * b1);
    }
    (boundary_points, abs(area) / 2)
}
