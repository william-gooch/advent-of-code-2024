use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("./input/day_8.txt");

pub fn day_8() {
    println!("--- Day 8 ---");

    let max_row = INPUT.lines().count() as isize;
    let max_col = INPUT.lines().next().unwrap().chars().count() as isize;

    let mut antennae: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let chars = INPUT.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, char)| {
                    char.is_alphanumeric()
                        .then_some((char, (row, col)))
                })
        })
        .for_each(|(char, (row, col))| {
            let a = antennae.entry(char).or_default();
            a.push((row as isize, col as isize));
        });

    let antinodes = antennae.iter()
        .flat_map(|(_, ants)| {
            ants.iter()
                .combinations(2)
                .flat_map(|v| {
                    let a = v[0];
                    let b = v[1];

                    let diff = (a.0 - b.0, a.1 - b.1);
                    let a_node = (a.0 + diff.0, a.1 + diff.1);
                    let b_node = (b.0 - diff.0, b.1 - diff.1);

                    [a_node, b_node]
                })
        })
        .filter(|&(row, col)| {
            row >= 0
                && col >= 0
                && row < max_row
                && col < max_col
        })
        .unique()
        .count();

    println!("Number of antinodes: {antinodes}");

    let antinodes = antennae.iter()
        .flat_map(|(_, ants)| {
            ants.iter()
                .combinations(2)
                .flat_map(|v| {
                    let a = v[0];
                    let b = v[1];

                    let diff = (a.0 - b.0, a.1 - b.1);

                    let a_nodes = (0..)
                        .map(move |i| {
                            let row = a.0 + (diff.0 * i);
                            let col = a.1 + (diff.1 * i);

                            (row, col)
                        })
                        .take_while(|&(row, col)| {
                            row >= 0
                                && col >= 0
                                && row < max_row
                                && col < max_col
                        });

                    let b_nodes = (0..)
                        .map(move |i| {
                            let row = b.0 - (diff.0 * i);
                            let col = b.1 - (diff.1 * i);

                            (row, col)
                        })
                        .take_while(|&(row, col)| {
                            row >= 0
                                && col >= 0
                                && row < max_row
                                && col < max_col
                        });

                    a_nodes.chain(b_nodes)
                })
        })
        .unique()
        .count();

    println!("Number of new antinodes: {antinodes}");
}
