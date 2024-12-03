use regex::Regex;


const INPUT: &str = include_str!("./input/day_3.txt");

pub fn day_3() {
    println!("--- Day 3 ---");

    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();

    let total: i32 = re.captures_iter(INPUT.trim())
        .map(|c| c.extract())
        .filter_map(|(_, [a, b])| Some(a.parse::<i32>().ok()? * b.parse::<i32>().ok()?))
        .sum();

    println!("sum of multiplications: {total}");

    let re = Regex::new("mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)").unwrap();

    let total_do_dont: i32 = re.captures_iter(INPUT.trim())
        .fold((0, true), |(acc, on), c| {
            let s = c.get(0).unwrap().as_str();
            if s.starts_with("mul") && on {
                let mut iter = c.iter().skip(1).take(2);
                let a = iter.next().unwrap().unwrap().as_str().parse::<i32>().unwrap();
                let b = iter.next().unwrap().unwrap().as_str().parse::<i32>().unwrap();
                (acc + (a * b), true)
            } else if s.starts_with("don't") {
                (acc, false)
            } else if s.starts_with("do") {
                (acc, true)
            } else {
                (acc, on)
            }
        }).0;

    println!("sum of multiplications: {total_do_dont}");
}
