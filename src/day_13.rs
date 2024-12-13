use nalgebra::{Matrix2, Vector2};
use regex::{Captures, Regex};

const INPUT: &str = include_str!("./input/day_13.txt");
// const INPUT: &str = r"Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279";

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: Vector2<i64>,
    button_b: Vector2<i64>,
    target:   Vector2<i64>,
}

const EPSILON: f64 = 0.001;

fn is_whole(f: f64) -> bool {
    (f - f.round()).abs() < EPSILON
}

impl ClawMachine {
    fn solve(&self) -> Option<u64> {
        let basis_matrix = nalgebra::Matrix2::from_columns(&[self.button_a, self.button_b]);
        let inverse_basis_matrix = basis_matrix.cast::<f64>().try_inverse().expect("Non-invertible");
        
        let coeffs = inverse_basis_matrix * self.target.cast::<f64>();
        println!("{coeffs:?}");
        let a_coeff = (is_whole(coeffs[0]) && coeffs[0] > 0.0).then_some(coeffs[0].round() as u64)?;
        let b_coeff = (is_whole(coeffs[1]) && coeffs[1] > 0.0).then_some(coeffs[1].round() as u64)?;
        println!("valid!");
        Some(3 * a_coeff + b_coeff)
    }
}

impl TryFrom<Captures<'_>> for ClawMachine {
    type Error = ();

    fn try_from(captures: Captures) -> Result<Self, Self::Error> {
        let a_x = captures.name("a_x").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let a_y = captures.name("a_y").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let b_x = captures.name("b_x").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let b_y = captures.name("b_y").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let t_x = captures.name("t_x").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let t_y = captures.name("t_y").ok_or(())?.as_str().parse().map_err(|_| ())?;

        Ok(Self {
            button_a: Vector2::new(a_x, a_y),
            button_b: Vector2::new(b_x, b_y),
            target:   Vector2::new(t_x, t_y),
        })
    }
}

pub fn day_13() {
    println!("--- Day 13 ---");

    let re = Regex::new(r"Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)\nButton B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)\nPrize: X=(?<t_x>\d+), Y=(?<t_y>\d+)")
        .expect("Couldn't make regex");

    let machines: Vec<ClawMachine> = re.captures_iter(INPUT)
        .map(|captures| {
            captures.try_into()
        })
        .collect::<Result<_, ()>>()
        .expect("Couldn't build claw machines");

    let total_cost = machines.iter()
        .filter_map(|machine| {
            machine.solve()
        })
        .sum::<u64>();

    println!("{total_cost}");

    let total_cost = machines.into_iter()
        .filter_map(|mut machine| {
            machine.target[0] = machine.target[0] + 10000000000000;
            machine.target[1] = machine.target[1] + 10000000000000;
            machine.solve()
        })
        .sum::<u64>();

    println!("{total_cost}");
}