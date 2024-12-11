use std::collections::HashMap;

const INPUT: &str = include_str!("./input/day_11.txt");
// const INPUT: &str = "125 17";

fn permute_stone(stone: u64) -> Vec<u64> {
    if stone == 0 { vec![1] }
    else if stone.ilog10() % 2 == 1 {
        let half_mag = 10_u64.pow((stone.ilog10() + 1) / 2);
        vec![stone / half_mag, stone % half_mag]
    } else {
        vec![stone * 2024]
    }
}

fn permute_stones(stones: &mut Vec<u64>) {
    *stones = stones.iter()
        .copied()
        .flat_map(|stone| {
            permute_stone(stone)
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CacheKey {
    value: u64,
    after_steps: usize,
}

impl CacheKey {
    fn calculate(&self, cache: &Cache) -> usize {
        if self.after_steps <= 0 { 1 }
        else {
            permute_stone(self.value).into_iter()
                .map(|v| CacheKey { value: v, after_steps: self.after_steps - 1 })
                .map(|key| {
                    if let Some(value) = cache.get(&key) {
                        value
                    } else {
                        cache.insert(key, Box::new(key.calculate(cache)))
                    }
                })
                .sum::<usize>()
        }
    }
}

type Cache = elsa::FrozenMap<CacheKey, Box<usize>>;

pub fn day_11() {
    println!("--- Day 11 ---");

    let cache: Cache = Default::default();

    let stones = INPUT.trim().split_whitespace()
        .filter_map(|s| {
            s.parse::<u64>().ok()
        })
        .collect::<Vec<_>>();

    let total_stones: usize = stones.iter()
        .map(|stone| CacheKey { value: *stone, after_steps: 75 })
        .map(|key| key.calculate(&cache))
        .sum();

    println!("Number of stones after 75 blinks: {}", total_stones);
}
