
const INPUT: &str = include_str!("./input/day_1.txt");

pub fn day_1() {
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

    let sum_diffs: u32 = Iterator::zip(list1.into_iter(), list2.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("Sum of differences: {sum_diffs}");
}
