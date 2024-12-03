use std::collections::BTreeMap;


const INPUT: &str = include_str!("./input/day_1.txt");

pub fn day_1() {
    println!("--- Day 1 ---");

    let (mut list1, mut list2): (Vec<_>, Vec<_>) = INPUT.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace().take(2);
            Some((
                parts.next()?
                    .parse::<u32>().ok()?,
                parts.next()?
                    .parse::<u32>().ok()?,
            ))
        })
        .unzip();

    list1.sort();
    list2.sort();

    let sum_diffs: u32 = Iterator::zip(list1.iter(), list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Sum of differences: {sum_diffs}");

    let mut counts: BTreeMap<u32, u32> = BTreeMap::new();
    list2.into_iter()
        .for_each(|i| {
            let entry = counts.entry(i)
                .or_insert(0);
            *entry += 1;
        });

    let similarity: u32 = list1.into_iter()
        .map(|i| i * counts.get(&i).copied().unwrap_or(0))
        .sum();

    println!("Similarity: {similarity}");
}
