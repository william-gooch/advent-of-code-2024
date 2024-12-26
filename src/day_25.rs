use itertools::{iproduct, Either, Itertools};

const INPUT: &str = include_str!("./input/day_25.txt");
// const INPUT: &str = r"#####
// .####
// .####
// .####
// .#.#.
// .#...
// .....

// #####
// ##.##
// .#.##
// ...##
// ...#.
// ...#.
// .....

// .....
// #....
// #....
// #...#
// #.#.#
// #.###
// #####

// .....
// .....
// #.#..
// ###..
// ###.#
// ###.#
// #####

// .....
// .....
// .....
// #....
// #.#..
// #.#.#
// #####";

pub fn day_25() {
    println!("--- Day 25 ---");

    let (locks, keys): (Vec<_>, Vec<_>) = INPUT.split("\n\n")
        .into_iter()
        .map(|chunk| {
            let is_lock = chunk.lines().next().unwrap().chars().all(|c| c == '#');
            let lines_iter: Box<dyn Iterator<Item = &str>> = if is_lock {
                Box::new(chunk.lines())
            } else {
                Box::new(chunk.lines().rev())
            };

            let lengths = lines_iter
                .skip(1)
                .fold(vec![0; 5], |acc, line| {
                    acc.into_iter()
                        .zip(line.chars())
                        .map(|(acc, c)| {
                            if c == '#' { acc + 1 } else { acc }
                        })
                        .collect::<Vec<usize>>()
                });
            
            (is_lock, lengths)
        })
        .partition_map(|(is_lock, length)| if is_lock { Either::Left(length) } else { Either::Right(length) });

    println!("{}, {}", locks.len(), keys.len());
    let pairs = iproduct!(locks, keys)
        .filter(|(lock, key)| {
            lock.into_iter().zip(key)
                .all(|(lock_len, key_len)| lock_len + key_len <= 5)
        })
        .count();
    println!("Number of pairs: {pairs}");
}