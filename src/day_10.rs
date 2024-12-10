use itertools::Itertools;
use ndarray::{Array2, Axis};

const INPUT: &str = include_str!("./input/day_10.txt");
// const INPUT: &str = r"89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732";

type Map = Array2<u32>;
type Pos = (usize, usize);

fn adjacent_pos((row, col): Pos, map: &Map) -> Vec<Pos> {
    let row = row as isize;
    let col = col as isize;

    [
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ]
        .into_iter()
        .filter_map(|(row, col)| {
            let row: usize = row.try_into().ok()?;
            let col: usize = col.try_into().ok()?;

            map.get((row, col))
                .is_some().then_some((row, col))
        })
        .collect::<Vec<_>>()
}

fn check_paths(pos: Pos, map: &Map) -> Vec<Pos> {
    let value = map[pos];
    if value == 9 { vec![pos] }
    else {
        adjacent_pos(pos, map)
            .iter()
            .filter(|&&adj_pos| map[adj_pos].checked_sub(value).is_some_and(|v| v == 1))
            .flat_map(|&adj_pos| check_paths(adj_pos, map))
            .collect::<Vec<_>>()
    }
}

pub fn day_10() {
    println!("--- Day 10 ---");

    let rows = INPUT.lines().count();
    let cols = INPUT.lines().next().unwrap().len();

    let flat_map = INPUT
        .lines()
        .flat_map(| l| {
            l.chars()
                .filter_map(|c| {
                    c.to_digit(10)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let map = Array2::from_shape_vec((rows, cols), flat_map).unwrap();

    let trailheads = map.indexed_iter()
        .filter(|(pos, &value)| value == 0)
        .map(|t| t.0)
        .collect::<Vec<_>>();

    let (total_unique_score, total_score): (usize, usize) = trailheads.into_iter()
        .map(|pos| {
            let paths = check_paths(pos, &map);

            let num_unique_paths = paths.iter().unique().count();
            let num_total_paths = paths.len();

            (num_unique_paths, num_total_paths)
        })
        .reduce(|(unique_a, total_a), (unique_b, total_b)| (unique_a + unique_b, total_a + total_b)).unwrap();

    println!("Total trailhead score (unique endpoints): {total_unique_score}");
    println!("Total trailhead score (all paths): {total_score}");
}
