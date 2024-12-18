use fxhash::FxHashMap as HashMap;
use std::collections::BinaryHeap;

use itertools::Itertools;
use ndarray::{Array2, Axis, Ix2};

const INPUT: &str = include_str!("./input/day_18.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    cost: u64,
    heuristic: u64,
    position: Ix2,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn possible_next_states(self, map: &Array2<bool>) -> impl Iterator<Item = State> + use<'_> {
        [
            Ix2(self.position[0] + 1, self.position[1]),
            Ix2(self.position[0].wrapping_sub(1), self.position[1]),
            Ix2(self.position[0], self.position[1] + 1),
            Ix2(self.position[0], self.position[1].wrapping_sub(1)),
        ].into_iter()
            .filter(|position| {
                position[0] < map.len_of(Axis(0)) &&
                position[1] < map.len_of(Axis(1)) &&
                !map[*position]
            })
            .map(move |position| {
                State {
                    position,
                    cost: self.cost + 1,
                    heuristic: (71 - position[0] as u64) + (71 - position[1] as u64)
                }
            })
    }
}

fn min_score(map: &Array2<bool>) -> Option<u64> {
    let mut lowest_score: HashMap<Ix2, u64> = HashMap::default();
    let mut p_queue = BinaryHeap::new();
    p_queue.push(State { cost: 0, position: Ix2(0, 0), heuristic: 71 + 71 });

    while let Some(state) = p_queue.pop() {
        if state.position == Ix2(70, 70) {
            return Some(state.cost);
        }

        let lowest = lowest_score.entry(state.position).or_insert(u64::MAX);
        if state.cost >= *lowest { continue; }
        else { *lowest = state.cost; }

        state.possible_next_states(map)
            .for_each(|next| {
                let lowest = lowest_score.entry(next.position).or_insert(u64::MAX);
                if next.cost <= *lowest {
                    p_queue.push(next.clone());
                }
            });
    }

    None
}

pub fn day_18() {
    println!("--- Day 18 ---");

    let map: Array2<bool> = Array2::default((71, 71));

    let positions = INPUT.lines()
        .map(|l| {
            let mut split = l.split(',');
            let (x, y) = l.split(',').take(2).map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
            Ix2(y, x)
        })
        .collect::<Vec<_>>();

    {
        let mut map = map.clone();    

        positions
            .iter()
            .take(1024)
            .for_each(|&pos| {
                map[pos] = true;
            });

        let min_score = min_score(&map);
        println!("Minimum distance: {min_score:?}");
    }

    let start = std::time::Instant::now();
    {
        let is_passable = |steps: usize| {
            let mut map = map.clone();

            positions.iter()
                .take(steps)
                .for_each(|&pos| {
                    map[pos] = true;
                });
            
            min_score(&map).is_some()
        };

        let steps = (0..positions.len()).collect::<Vec<_>>();
        let partition_point = steps.partition_point(|&steps| is_passable(steps));

        let first_impassable = positions[partition_point - 1];
        let first_impassable = (first_impassable[1], first_impassable[0]);

        println!("First byte to block off path: {:?}", first_impassable);
    }
    let elapsed = start.elapsed();
    println!("Took {elapsed:?}");
}