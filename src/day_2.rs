
const INPUT: &str = include_str!("./input/day_2.txt");

pub fn day_2() {
    let safe_reports = INPUT
        .trim()
        .lines()
        .map(|line| { // convert each line to a vec of integers, splitting by whitespace
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            let diffs = report.iter()
                .zip(report.iter().skip(1))
                .map(|(a, b)| a - b);

            let is_monotonic = diffs.clone().all(|d| d <= 0) || diffs.clone().all(|d| d >= 0);
            let is_within_range = diffs.clone().all(|d| d.abs() >= 1 && d.abs() <= 3);

            is_monotonic && is_within_range
        })
        .count();

    println!("safe reports: {safe_reports}");
}
