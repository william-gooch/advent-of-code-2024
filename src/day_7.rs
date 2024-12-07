use std::{num::ParseIntError};

const INPUT: &str = include_str!("./input/day_7.txt");

fn concat(a: u64, b: u64) -> u64 {
    let num_digits_b = b.ilog10() + 1;
    let a_mult = a * 10u64.pow(num_digits_b);
    a_mult + b
}

#[derive(Debug)]
struct Equation {
    pub result: u64,
    pub values: Vec<u64>,
}

impl Equation {
    fn calibrate_values(target: u64, accum: u64, values: &[u64]) -> bool {
        if values.len() == 0 {
            (accum == target)
        } else if (accum > target) {
            false
        } else {
            Equation::calibrate_values(target, accum + values[0], &values[1..]) ||
            Equation::calibrate_values(target, accum * values[0], &values[1..])
        }
    }

    fn calibrate_values_with_concat(target: u64, accum: u64, values: &[u64]) -> bool {
        if values.len() == 0 {
            (accum == target)
        } else if (accum > target) {
            false
        } else {
            Equation::calibrate_values_with_concat(target, accum + values[0], &values[1..]) ||
            Equation::calibrate_values_with_concat(target, accum * values[0], &values[1..]) ||
            Equation::calibrate_values_with_concat(target, concat(accum, values[0]), &values[1..])
        }
    }
    
    pub fn calibrate(&self) -> bool {
        Equation::calibrate_values(self.result, self.values[0], &self.values[1..])
    }

    pub fn calibrate_with_concat(&self) -> bool {
        Equation::calibrate_values_with_concat(self.result, self.values[0], &self.values[1..])
    }
}

pub fn day_7() {
    println!("--- Day 7 ---");

    let eqns = INPUT.lines()
        .map(|line| -> Result<Equation, String> {
            let (result_str, value_str) = line.split_once(": ").ok_or_else(|| "No ': ' found in line".to_owned())?;
            let result: u64 = result_str.parse().map_err(|err: ParseIntError| err.to_string())?;
            let values: Vec<u64> = value_str.split(' ')
                .map(|c| c.parse())
                .try_collect()
                .map_err(|err: ParseIntError| err.to_string())?;
            (values.len() >= 2).then(|| ()).ok_or_else(|| "Less than two values provided".to_owned())?;

            Ok(Equation { result, values })
        })
        .try_collect::<Vec<_>>()
        .unwrap();

    let total_calibration_result = eqns
        .iter()
        .filter(|eqn| eqn.calibrate())
        .map(|eqn| eqn.result)
        .sum::<u64>();

    println!("Total Calibration Result: {total_calibration_result}");

    let total_calibration_result_with_concat = eqns
        .iter()
        .filter(|eqn| eqn.calibrate_with_concat())
        .map(|eqn| eqn.result)
        .sum::<u64>();

    println!("Total Calibration Result: {total_calibration_result_with_concat}");
}
