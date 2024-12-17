use std::rc::Rc;

use itertools::Itertools;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use regex::Regex;

const INPUT: &str = include_str!("./input/day_17.txt");
// const INPUT: &str = r"Register A: 2024
// Register B: 0
// Register C: 0

// Program: 0,3,5,4,3,0";

#[derive(FromPrimitive, Debug, Clone, Copy)]
enum Opcode {
    Adv = 0, // A <- trunc(A / pow(2, combo_op))
    Bxl = 1, // B <- B ^ lit_op
    Bst = 2, // B <- combo_op % 8
    Jnz = 3, // IP <- (A == 0 ? IP : lit_op)
    Bxc = 4, // B <- B ^ C
    Out = 5, // output combo_op % 8
    Bdv = 6, // B <- trunc(A / pow(2, combo_op))
    Cdv = 7, // C <- trunc(A / pow(2, combo_op))
}

enum CycleResult {
    NoOutput,
    Output(u8),
    Done,
}

#[derive(Clone)]
struct State {
    a: isize,
    b: isize,
    c: isize,
    ip: usize,

    instructions: Rc<[u8]>,
}

impl State {
    fn combo_op(&self, operand: u8) -> isize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo_op")
        }
    }

    fn cycle(&mut self) -> CycleResult {
        let opcode = self.instructions[self.ip];
        let operand = self.instructions[self.ip + 1];

        let mut did_jump = false;
        let mut output = None;

        use Opcode::*;
        match Opcode::from_u8(opcode) {
            Some(Adv) => {
                self.a = self.a / isize::pow(2, self.combo_op(operand) as u32)
            }, // A <- trunc(A / pow(2, combo_op))
            Some(Bxl) => {
                self.b = self.b ^ (operand as isize)
            }, // B <- B ^ lit_op
            Some(Bst) => {
                self.b = self.combo_op(operand) % 8
            }, // B <- combo_op % 8
            Some(Jnz) => {
                if self.a != 0 {
                    self.ip = operand as usize;
                    did_jump = true
                }
            }, // IP <- (A == 0 ? IP : lit_op)
            Some(Bxc) => {
                self.b = self.b ^ self.c
            }, // B <- B ^ C
            Some(Out) => {
                output.replace((self.combo_op(operand) % 8) as u8);
            }, // output combo_op % 8
            Some(Bdv) => {
                self.b = self.a / isize::pow(2, self.combo_op(operand) as u32)
            }, // B <- trunc(A / pow(2, combo_op))
            Some(Cdv) => {
                self.c = self.a / isize::pow(2, self.combo_op(operand) as u32)
            }, // C <- trunc(A / pow(2, combo_op))
            None => panic!("invalid opcode"),
        };

        // println!("{: >2}: {:?}, {operand} // A: {: >8} // B: {: >8} // C: {: >8}", self.ip, Opcode::from_u8(opcode), self.a, self.b, self.c);

        if !did_jump {
            self.ip += 2;
        }

        if self.ip >= self.instructions.len() {
            return CycleResult::Done;
        }

        output.map_or(CycleResult::NoOutput, |o| CycleResult::Output(o))
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Default::default();
        loop {
            match self.cycle() {
                CycleResult::Done => break,
                CycleResult::Output(o) => output.push(o),
                _ => ()
            }        
        }

        output
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"Register A: (?<a>\d+)\nRegister B: (?<b>\d+)\nRegister C: (?<c>\d+)\n\nProgram: (?<program>(?:\d,?)+)").unwrap();
        let captures = regex.captures(value).expect("No match found");

        let program = captures.name("program").unwrap()
            .as_str().split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>();

        Self {
            a: captures.name("a").unwrap().as_str().parse().unwrap(),
            b: captures.name("b").unwrap().as_str().parse().unwrap(),
            c: captures.name("c").unwrap().as_str().parse().unwrap(),
            ip: 0,
            instructions: program.into(),
        }
    }
}

fn print_octets(a: isize) {
    if a == 0 { print!("0 "); return; }
    let mut curr = a;
    while curr > 0 {
        let n = curr % 8;
        curr = curr >> 3;
        print!("{n} ");
    }
}

fn prog_loop(a: isize) -> u8 {
    let mut b = a % 8;
    b ^= 5;
    let mut c = a >> b;
    b ^= 6;
    b ^= c;
    let output = b % 8;

    output as u8
}

fn prog_find(target_out: u8, current_a: isize) -> Vec<isize> {
    println!("Previous: {current_a}");
    println!("Target: {target_out}");
    let attempted_a = current_a << 3;
    (0..8)
        .filter_map(|i| {
            let out = prog_loop(attempted_a + i);
            print_octets(attempted_a + i);
            println!("// {out}");
            (out == target_out).then_some(attempted_a + i)
        })
        .collect::<Vec<_>>()
}

fn prog_recurse(target_out: &[u8], current_a: isize) -> Option<isize> {
    if target_out.len() == 0 { return Some(current_a) }

    let options = prog_find(target_out[0], current_a);
    println!("found options!: {options:?}");
    options.iter()
        .find_map(|option| prog_recurse(&target_out[1..], *option))
}

pub fn day_17() {
    println!("--- Day 17 ---");

    let state: State = INPUT.into();

    {
        let mut state = state.clone();
        let output = state.run().into_iter().map(|o| o.to_string()).join(",");
        println!("Output: {output}");
    }

    let output_rev = state.instructions.iter().rev().copied().collect::<Vec<_>>();
    let valid_a = prog_recurse(&output_rev[..], 0);
    println!("valid a: {:?}", valid_a);
}