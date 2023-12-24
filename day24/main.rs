use itertools::Itertools;
use num::range;
use z3::ast::{Ast, Int};

// wow (ok I learnt this from AxlLind)

#[derive(Debug, Copy, Clone)]
struct Hail {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hail {
    fn intersection_point(&self, other: &Self) -> Option<(f64, f64)> {
        // 2d x and y only
        let (x1, y1, _) = self.position;
        let (vx1, vy1, _) = self.velocity;

        let (x2, y2, _) = other.position;
        let (vx2, vy2, _) = other.velocity;

        let m1 = vy1 / vx1; // dy / dx
        let m2 = vy2 / vx2; // dy / dx

        if (m2 - m1).abs() < f64::EPSILON {
            return None; // parallel
        }

        // search from stack overflow
        let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
        let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
        Some((x, y))
    }
}

fn parse(line: &str) -> Hail {
    let (position, velocity) = line.split_once('@').unwrap();
    let position: Vec<f64> = position.split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect();
    let velocity: Vec<f64> = velocity.split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect();
    Hail {
        position: (position[0], position[1], position[2]),
        velocity: (velocity[0], velocity[1], velocity[2]),
    }
}

fn find_total_intersecting(hails: &Vec<Hail>) -> usize {
    let total_area = 200_000_000_000_000.0..=400_000_000_000_000.0;
    hails.iter()
        .tuple_combinations()
        .filter(|(&h1, &h2)| {
            let Some((x, y)) = h1.intersection_point(&h2) else { return false; };
            // check future
            if h1.velocity.0.signum() != (x - h1.position.0).signum() || h2.velocity.0.signum() != (x - h2.position.0).signum() {
                return false; // in the past
            }

            total_area.contains(&x) && total_area.contains(&y)
        })
        .count()
}

// 2nd part taken from axllind, cos the library is so beautiful and im too dumb
fn part2(hails: &Vec<Hail>) -> i64 {
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] = ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0);
    for (i, h) in hails.iter().enumerate() {
        let (x, y, z) = h.position;
        let (dx, dy, dz) = h.velocity;
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as i64));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let all_hails: Vec<Hail> = input.lines().map(parse).collect();
    let result = find_total_intersecting(&all_hails);
    let result = part2(&all_hails);
    println!("{:?}", result);
    // let y = 2;
    // let x = 1 * &y;
}