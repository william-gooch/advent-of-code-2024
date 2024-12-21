use std::{borrow::Borrow, iter::{once, repeat}};

use fxhash::FxHashMap;
use itertools::{iproduct, Itertools};
use ndarray::Ix2;

const INPUT: &str = include_str!("./input/day_21.txt");
// const INPUT: &str = r"029A
// 980A
// 179A
// 456A
// 379A";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    MoveUp,
    MoveRight,
    MoveDown,
    MoveLeft,
    Push,
}

const OPERATIONS: [Operation; 5] = [Operation::MoveUp, Operation::MoveRight, Operation::MoveDown, Operation::MoveLeft, Operation::Push];

impl From<Operation> for Ix2 {
    fn from(value: Operation) -> Self {
        match value {
            Operation::MoveUp => Ix2(0, 1),
            Operation::MoveRight => Ix2(1, 2),
            Operation::MoveDown => Ix2(1, 1),
            Operation::MoveLeft => Ix2(1, 0),
            Operation::Push => Ix2(0, 2),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Button(char);

impl From<Button> for Ix2 {
    fn from(value: Button) -> Self {
        match value {
            Button('7') => Ix2(0, 0),
            Button('8') => Ix2(0, 1),
            Button('9') => Ix2(0, 2),
            Button('4') => Ix2(1, 0),
            Button('5') => Ix2(1, 1),
            Button('6') => Ix2(1, 2),
            Button('1') => Ix2(2, 0),
            Button('2') => Ix2(2, 1),
            Button('3') => Ix2(2, 2),
            Button('0') => Ix2(3, 1),
            Button('A') => Ix2(3, 2),
            _ => panic!("invalid character!"),
        }
    }
}

fn numeric_shortest_path(from: Ix2, to: Ix2) -> Vec<impl Iterator<Item = Operation> + Clone> {
    let y_diff = to[0].checked_signed_diff(from[0]).unwrap();
    let x_diff = to[1].checked_signed_diff(from[1]).unwrap();

    let x_steps = 
        if x_diff >= 0 {
            repeat(Operation::MoveRight).take(x_diff.unsigned_abs())
        } else {
            repeat(Operation::MoveLeft).take(x_diff.unsigned_abs())
        };
    let y_steps = 
        if y_diff >= 0 {
            repeat(Operation::MoveDown).take(y_diff.unsigned_abs())
        } else {
            repeat(Operation::MoveUp).take(y_diff.unsigned_abs())
        };

    if Ix2(from[0].checked_add_signed(y_diff).unwrap(), from[1]) == Ix2(3, 0) {
        vec![x_steps.chain(y_steps)]
    } else if Ix2(from[0], from[1].checked_add_signed(x_diff).unwrap()) == Ix2(3, 0) {
        vec![y_steps.chain(x_steps)]
    } else if y_diff != 0 && x_diff != 0 {
        vec![x_steps.clone().chain(y_steps.clone()), y_steps.chain(x_steps)]
    } else {
        vec![y_steps.chain(x_steps)]
    }
}

fn directional_shortest_path(from: Ix2, to: Ix2) -> Vec<impl Iterator<Item = Operation> + Clone> {
    let y_diff = to[0].checked_signed_diff(from[0]).unwrap();
    let x_diff = to[1].checked_signed_diff(from[1]).unwrap();

    let x_steps = 
        if x_diff >= 0 {
            repeat(Operation::MoveRight).take(x_diff.unsigned_abs())
        } else {
            repeat(Operation::MoveLeft).take(x_diff.unsigned_abs())
        };
    let y_steps = 
        if y_diff >= 0 {
            repeat(Operation::MoveDown).take(y_diff.unsigned_abs())
        } else {
            repeat(Operation::MoveUp).take(y_diff.unsigned_abs())
        };

    if Ix2(from[0].checked_add_signed(y_diff).unwrap(), from[1]) == Ix2(0, 0) {
        vec![x_steps.chain(y_steps)]
    } else if Ix2(from[0], from[1].checked_add_signed(x_diff).unwrap()) == Ix2(0, 0) {
        vec![y_steps.chain(x_steps)]
    } else if y_diff != 0 && x_diff != 0 {
        vec![x_steps.clone().chain(y_steps.clone()), y_steps.chain(x_steps)]
    } else {
        vec![y_steps.chain(x_steps)]
    }
}

fn initial_keypad_costs() -> FxHashMap<(Operation, Operation), u64> {
    iproduct!(OPERATIONS, OPERATIONS)
        .map(|tuple| (tuple, 1))
        .collect()
}

fn precompute_keypad_costs(upper_level: FxHashMap<(Operation, Operation), u64>) -> FxHashMap<(Operation, Operation), u64> {
    iproduct!(OPERATIONS, OPERATIONS)
        .map(|(from, to)| {
            let min_cost = directional_shortest_path(from.into(), to.into())
                .into_iter()
                .map(|path| {
                    let mut current_op = Operation::Push;
                    path.chain(once(Operation::Push)).map(|op| {
                        let cost = upper_level[&(current_op, op)];
                        current_op = op;
                        cost
                    }).sum::<u64>()
                })
                .min().unwrap();

            ((from, to), min_cost)
        })
        .collect()
}

fn compute_numeric_costs(path: impl IntoIterator<Item = (Button, Button)>, upper_level: &FxHashMap<(Operation, Operation), u64>) -> u64 {
    path.into_iter()
        .map(|(from, to)| {
            let min_cost = numeric_shortest_path(from.into(), to.into())
                .into_iter()
                .map(|path| {
                    let mut current_op = Operation::Push;
                    path.chain(once(Operation::Push)).map(|op| {
                        let cost = upper_level[&(current_op, op)];
                        current_op = op;
                        cost
                    }).sum::<u64>()
                })
                .min().unwrap();

            min_cost
        })
        .sum()
}

pub fn day_21() {
    println!("--- Day 21 ---");

    let initial = initial_keypad_costs();
    let mut dpad = initial;
    for _ in 0..25 {
        dpad = precompute_keypad_costs(dpad);
    }

    let sum_complexity = INPUT.lines()
        .map(|code| {
            let buttons = code.chars().map(Button).collect::<Vec<_>>();

            let path = once(Button('A')).chain(buttons.into_iter()).tuple_windows();
            let num_steps = compute_numeric_costs(path, &dpad);

            let numeric_code = code.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().expect("Couldn't parse numeric code.");

            println!("{code}: {num_steps} x {numeric_code}");
            num_steps * numeric_code
        })
        .sum::<u64>();
    println!("Total complexity (pt. 1): {sum_complexity}");
}