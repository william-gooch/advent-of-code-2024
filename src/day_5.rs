use std::{cmp::Ordering, collections::BTreeSet};

use itertools::Itertools;

const INPUT: &str = include_str!("./input/day_5.txt");

pub fn day_5() {
    println!("--- Day 5 ---");

    let ordering_rules = INPUT.lines()
        .take_while(|s| s.trim().len() > 0)
        .filter_map(|s| {
            let mut parts = s.split("|");
            Some((parts.next()?.parse::<u32>().ok()?, parts.next()?.parse::<u32>().ok()?))
        })
        .collect::<BTreeSet<_>>();

    let pages_to_produce = INPUT.lines()
        .skip_while(|s| s.trim().len() > 0)
        .skip(1)
        .map(|s| {
            s.split(",")
                .filter_map(|part| part.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let sum_of_valid_middle = pages_to_produce.clone().into_iter()
        .filter(|pages| {
            pages.iter().copied().tuple_combinations::<(u32, u32)>()
                .all(|(first, second)| {
                    !ordering_rules.contains(&(second, first))
                })
        })
        .map(|pages| {
            let middle_idx = (pages.len() / 2);
            pages[middle_idx]
        })
        .sum::<u32>();

    println!("sum of valid middle pages: {sum_of_valid_middle:?}");

    let sum_of_corrected_middle = pages_to_produce.iter()
        .filter(|pages| {
            !pages.iter().copied().tuple_combinations::<(u32, u32)>()
                .all(|(first, second)| {
                    !ordering_rules.contains(&(second, first))
                })
        })
        .map(|pages| {
            let mut pages = pages.clone();
            pages.sort_by(|&a, &b| {
                if ordering_rules.contains(&(a, b)) {
                    Ordering::Less
                } else if ordering_rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            pages
        })
        .map(|pages| {
            let middle_idx = (pages.len() / 2);
            pages[middle_idx]
        })
        .sum::<u32>();

    println!("sum of corrected invalid middle pages: {sum_of_corrected_middle:?}");
}
