use std::collections::{HashMap, VecDeque};

use ndarray::Array2;

const INPUT: &str = include_str!("./input/day_12.txt");

type Pos = (usize, usize);

fn adjacent_pos<T>((row, col): Pos, map: &Array2<T>) -> Vec<Pos> {
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

fn diagonal_pos<T>((row, col): Pos, map: &Array2<T>) -> Vec<Pos> {
    let row = row as isize;
    let col = col as isize;

    [
        (row - 1, col - 1),
        (row + 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col + 1),
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

pub fn day_12() {
    println!("--- Day 12 ---");

    let rows = INPUT.lines().count();
    let cols = INPUT.lines().next().unwrap().len();

    let flat_map = INPUT.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();

    let map = Array2::from_shape_vec((rows, cols), flat_map).unwrap();

    {
        let mut visited_map = Array2::from_shape_vec((rows, cols), vec![false; (rows * cols)]).unwrap();
        let mut prices: Vec<usize> = Default::default();
        map.indexed_iter()
            .for_each(|(seed_pos, &c)| {
                if visited_map[seed_pos] { return }

                let mut queue = VecDeque::new();
                queue.push_back(seed_pos);

                let mut area = 0;
                let mut perimeter = 0;
                while let Some(pos) = queue.pop_front() {
                    if visited_map[pos] { continue }

                    let len = queue.len();
                    let adjs = adjacent_pos(pos, &map)
                        .into_iter()
                        .filter(|adj_pos| map[*adj_pos] == c)
                        .collect_into(&mut queue);
                    let num_adjs = queue.len() - len;

                    area += 1;
                    perimeter += (4 - num_adjs);
                    visited_map[pos] = true;
                }

                prices.push(area * perimeter);
            });

        let total_price: usize = prices.into_iter().sum();
        println!("Total price: {total_price}");
    }

    {
        let mut visited_map = Array2::from_shape_vec((rows, cols), vec![false; (rows * cols)]).unwrap();
        let mut prices: Vec<usize> = Default::default();

        map.indexed_iter()
            .for_each(|(seed_pos, &c)| {
                if visited_map[seed_pos] { return }

                let mut queue = VecDeque::new();
                queue.push_back(seed_pos);

                let mut area = 0;
                let mut interior_angles = 0;
                let mut exterior_angles = 0;
                while let Some(pos) = queue.pop_front() {
                    if visited_map[pos] { continue }

                    let len = queue.len();
                    let adjs = adjacent_pos(pos, &map)
                        .into_iter()
                        .filter(|adj_pos| map[*adj_pos] == c)
                        .collect::<Vec<_>>();
                    queue.extend(&adjs);

                    interior_angles += match adjs.len() {
                        2 => {
                            if adjs[0].0.abs_diff(adjs[1].0) == 0
                            || adjs[0].1.abs_diff(adjs[1].1) == 0 {
                                0
                            } else { 1 }
                        },
                        1 => 2,
                        0 => 4,
                        _ => 0,
                    };

                    let num_diags = diagonal_pos(pos, &map)
                        .into_iter()
                        .filter(|diag_pos| map[*diag_pos] != c)
                        .filter(|&(row, col)| {
                            map[(row, pos.1)] == c && map[(pos.0, col)] == c
                        })
                        .count();

                    exterior_angles += num_diags;

                    area += 1;
                    visited_map[pos] = true;
                }

                prices.push(area * (interior_angles + exterior_angles));
            });

        let total_price: usize = prices.into_iter().sum();
        println!("Total price (with discount): {total_price}");
    }
}