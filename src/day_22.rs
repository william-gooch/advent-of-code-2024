const INPUT: &str = include_str!("./input/day_22.txt");

#[inline]
const fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

#[inline]
const fn prune(a: u64) -> u64 {
    a % 16777216
}

#[inline]
const fn hash_cycle(a: u64) -> u64 {
    let a = prune(mix(a, a * 64));
    let a = prune(mix(a, a / 32));
    let a = prune(mix(a, a * 2048));

    a
}

fn hash(a: u64, cycles: usize) -> u64 {
    (0..cycles)
        .fold(a, |a, _| hash_cycle(a))
}

pub fn day_22() {
    println!("--- Day 22 ---");

    let seeds = INPUT.lines()
        .map(|l| l.parse())
        .try_collect::<Vec<u64>>()
        .expect("Couldn't parse number");

    let sum_hashes = seeds.iter()
        .map(|&seed| hash(seed, 2000))
        .sum::<u64>();
    println!("Sum of hash values: {sum_hashes}");
}