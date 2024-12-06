const INPUT: &str = include_str!("./input/day_2.txt");

pub fn day_2() {
    println!("--- Day 2 ---");

    let reports = INPUT
        .trim()
        .lines()
        .map(|line| {
            // convert each line to a vec of integers, splitting by whitespace
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let is_safe = |report: &Vec<i32>| {
        let diffs = report.iter().zip(report.iter().skip(1)).map(|(a, b)| a - b);

        let is_monotonic = diffs.clone().all(|d| d <= 0) || diffs.clone().all(|d| d >= 0);
        let is_within_range = diffs.clone().all(|d| d.abs() >= 1 && d.abs() <= 3);

        is_monotonic && is_within_range
    };

    let safe_reports = reports.iter().filter(|report| is_safe(report)).count();

    println!("safe reports: {safe_reports}");

    let newly_safe_reports = reports
        .iter()
        .filter(|report| !is_safe(report))
        .filter(|report| {
            report
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mut modified = (*report).clone();
                    modified.remove(i);
                    modified
                })
                .any(|report| is_safe(&report))
        })
        .count();

    println!("newly safe reports: {}", newly_safe_reports + safe_reports);
}
