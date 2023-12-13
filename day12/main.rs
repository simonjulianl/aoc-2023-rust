mod static_vec;

type Vec<T> = static_vec::StaticVec<T, 128>;
type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Spring {
    #[default]
    Ok,
    Broken,
    Unknown,
}

impl From<u8> for Spring {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Ok,
            b'#' => Self::Broken,
            b'?' => Self::Unknown,
            _ => panic!("oh no"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
enum Outcome {
    #[default]
    None,
    Invalid,
    Valid(i64),
}

impl Outcome {
    fn unwrap_or(&self, value: i64) -> i64 {
        if let Self::Valid(outcome) = self {
            *outcome
        } else {
            value
        }
    }
}

fn arrangements_memoized(springs: &[Spring], lens: &[usize]) -> i64 {
    let mut m = Matrix::of(Vec::of(Outcome::None));
    arrangements(springs, lens, &mut m).unwrap_or(0)
}

fn place(len: usize, springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    if len > springs.len() {
        Outcome::Invalid
    } else if springs[..len].iter().any(|s| *s == Spring::Ok) {
        Outcome::Invalid
    } else if len >= springs.len() {
        // this is the right end
        // println!("{:?} {len}", springs);
        arrangements(&springs[len..], lens, m)
    } else if springs[len] == Spring::Broken {
        Outcome::Invalid
    } else {
        arrangements(&springs[len + 1..], lens, m)
    }
}

fn arrangements(springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    if let memo @ (Outcome::Valid(_) | Outcome::Invalid) = m[springs.len()][lens.len()] {
        return memo;
    }

    let outcome = match (springs.iter().next(), lens.iter().next()) {
        (None, None) => Outcome::Valid(1),
        (None, Some(_)) => Outcome::Invalid,
        (Some(Spring::Ok), _) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Broken), None) => Outcome::Invalid,
        (Some(Spring::Broken), Some(len)) => place(*len, springs, &lens[1..], m),
        (Some(Spring::Unknown), None) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Unknown), Some(len)) => {
            let here = place(*len, springs, &lens[1..], m).unwrap_or(0);
            let there = arrangements(&springs[1..], lens, m).unwrap_or(0);
            Outcome::Valid(here + there)
        }
    };
    m[springs.len()][lens.len()] = outcome;
    outcome
}

fn parse(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs, lens) = line.split_once(' ').unwrap();
    let springs = springs.bytes().map(Spring::from).collect();
    let lens = lens.split(',').map(|len| len.parse::<usize>().unwrap()).collect();
    (springs, lens)
}

fn expand(copy: i64, s: &mut Vec<Spring>, l: &mut Vec<usize>) {
    let springs_len = s.len();
    let lens_len = l.len();
    for _ in 1..copy {
        s.push(Spring::Unknown);
        for j in 0..springs_len {
            s.push(s[j]);
        }
        for j in 0..lens_len {
            l.push(l[j]);
        }
    }
}

fn sum_arrangements(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let (mut springs, mut lens) = parse(line);
        expand(5, &mut springs, &mut lens);
        sum += arrangements_memoized(&springs[..], &lens[..]);
    }

    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", input);
    // let x: &[i64] = &[1];
    let x = sum_arrangements(input);
    println!("{x}");
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn run() {}
// }
