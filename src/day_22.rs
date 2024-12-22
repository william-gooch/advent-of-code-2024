use fxhash::{FxHashMap, FxHashSet};
use itertools::{iproduct, Itertools};
use ndarray::Array2;
use num_traits::ToPrimitive;

const INPUT: &str = include_str!("./input/day_22.txt");
// const INPUT: &str = r"1
// 2
// 3
// 2024";

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

fn all_hashes(a: u64) -> impl Iterator<Item = u64> {
    let mut current_hash = a;
    (0..)
        .map(move |_| {
            current_hash = hash_cycle(current_hash);
            current_hash
        })
}

fn all_prices(a: u64) -> impl Iterator<Item = u8> {
    all_hashes(a)
        .map(|a| (a % 10).to_u8().unwrap())
}

fn price_differences(a: u64) -> impl Iterator<Item = (u8, i8)> {
    all_prices(a)
        .tuple_windows()
        .map(|(a, b)| (b, a.checked_signed_diff(b).unwrap()))
}

fn all_price_differences(seeds: impl IntoIterator<Item = u64> + Clone) -> FxHashMap<[i8; 4], u64> {
    let num_seeds = seeds.clone().into_iter().count();
    let mut cache = FxHashMap::default();
    seeds.into_iter()
        .for_each(|seed| {
            let mut already_seen = FxHashSet::default();
            let cache = price_differences(seed).take(2000)
                .tuple_windows()
                .filter_map(move |(a, b, c, d)| {
                    let changes = [a.1, b.1, c.1, d.1];
                    if !already_seen.contains(&changes) {
                        already_seen.insert(changes.clone());
                        // println!("{seed}: {changes:?} {}", d.0);
                        Some((changes, d.0))
                    } else { None }
                })
                .for_each(|(key, cost)| {
                    let entry = cache.entry(key).or_insert(0);
                    *entry += cost as u64;
                });
        });

    cache
}

// fn change_combinations() -> impl Iterator<Item = [i8; 4]> {
//     iproduct!(
//         -9..=9,
//         -9..=9,
//         -9..=9,
//         -9..=9,
//     )
//         .map(|(a, b, c, d)| [a, b, c, d])
// }

pub fn day_22() {
    println!("--- Day 22 ---");

    let seeds: Vec<u64> = INPUT.lines()
        .map(|l| l.parse())
        .try_collect()
        .expect("Couldn't parse number");

    let sum_hashes = seeds.iter()
        .map(|&seed| hash(seed, 2000))
        .sum::<u64>();
    println!("Sum of hash values: {sum_hashes}");

    let price_diffs = all_price_differences(seeds);
    let most_bananas = price_diffs
        .into_iter()
        .max_by_key(|(key, cost)| *cost).unwrap();
    println!("Most bananas possible: {most_bananas:?}");
}