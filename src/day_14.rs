use std::collections::HashMap;

use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use ndarray::{Array2, Axis};
use regex::{Captures, Regex};

const INPUT: &str = include_str!("./input/day_14.txt");
// const INPUT: &str = r"p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3";

struct Robot {
    position: Vector2<u32>,
    velocity: Vector2<i32>,
}

impl Robot {
    fn position_after(&self, steps: u32, map_size: Vector2<u32>) -> Vector2<u32> {
        let mut new_position = (self.position.cast::<i32>()) + (steps as i32) * self.velocity;
        new_position[0] = new_position[0].rem_euclid(map_size[0] as i32);
        new_position[1] = new_position[1].rem_euclid(map_size[1] as i32);
        new_position.try_cast().expect("negative?")
    }
}

impl TryFrom<Captures<'_>> for Robot {
    type Error = ();

    fn try_from(captures: Captures) -> Result<Self, Self::Error> {
        let p_x = captures.name("p_x").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let p_y = captures.name("p_y").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let v_x = captures.name("v_x").ok_or(())?.as_str().parse().map_err(|_| ())?;
        let v_y = captures.name("v_y").ok_or(())?.as_str().parse().map_err(|_| ())?;

        Ok(Self {
            position: Vector2::new(p_x, p_y),
            velocity: Vector2::new(v_x, v_y),
        })
    }
}

fn positions_to_map(positions: &[Vector2<u32>], map_size: Vector2<u32>) -> Array2<u32> {
    let mut map: Array2<u32> = Array2::zeros((map_size[1] as usize, map_size[0] as usize));
    positions.iter()
        .for_each(|pos| {
            map[(pos[1] as usize, pos[0] as usize)] += 1;
        });
    
    map
}

fn christmas_tree_heuristic(map: &Array2<u32>) -> usize {
    let max_contiguous: usize = map.axis_iter(Axis(0))
        .filter_map(|row| {
            row.iter()
                .chunk_by(|&&i| i > 0)
                .into_iter()
                .filter_map(|c| {
                    (c.0).then_some(c.1.count())
                })
                .max()
        })
        .sum();

    max_contiguous
}

fn print_map(map: &Array2<u32>) {
    map.rows().into_iter()
        .for_each(|row| {
            row.into_iter().for_each(|v| print!("{v}"));
            println!("");
        })
}

pub fn day_14() {
    println!("--- Day 14 ---");

    let re = Regex::new(r"p=(?<p_x>-?\d+),(?<p_y>-?\d+) v=(?<v_x>-?\d+),(?<v_y>-?\d+)")
        .expect("Couldn't make regex");

    let robots: Vec<Robot> = re.captures_iter(INPUT)
        .map(|captures| {
            captures.try_into()
        })
        .collect::<Result<_, ()>>()
        .expect("Couldn't build robots");

    let map_size = Vector2::new(101, 103);
    // let map_size = Vector2::new(11, 7);
    let positions = robots.iter()
        .map(|robot| {
            robot.position_after(100, map_size)
        })
        .collect::<Vec<_>>();

    let map = positions_to_map(&positions, map_size);
    print_map(&map);
    
    let quadrants = positions.into_iter()
        .filter_map(|position| {
            let x_pivot = map_size[0] / 2;
            let y_pivot = map_size[1] / 2;
            let x_half = (position[0] != x_pivot).then_some(position[0] < x_pivot)?;
            let y_half = (position[1] != y_pivot).then_some(position[1] < y_pivot)?;

            Some(((x_half, y_half), position))
        })
        .into_group_map();

    let safety_factor: usize = quadrants.values()
        .map(|v| v.len())
        .product();

    println!("Safety factor: {safety_factor}");

    let (likely_tree, likelihood, map) = (0..10000)
        .map(|i| {
            let positions = robots.iter()
                .map(|robot| {
                    robot.position_after(i, map_size)
                })
                .collect::<Vec<_>>();
            let map = positions_to_map(&positions, map_size);
            (i, christmas_tree_heuristic(&map), map)
        })
        .max_by_key(|(i, h, m)| *h)
        .expect("No maximum found");

    print_map(&map);
    println!("Most likely tree: {likely_tree} (confidence: {likelihood})");
}