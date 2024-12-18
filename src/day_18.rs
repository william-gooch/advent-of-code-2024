use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use ndarray::{Array2, Axis, Ix2};

const INPUT: &str = include_str!("./input/day_18.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn possible_next_states(&self, map: &Array2<bool>) -> Vec<State> {
        vec![
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
            .map(|position| {
                State {
                    position,
                    cost: self.cost + 1,
                    heuristic: (71 - position[0] as u64) + (71 - position[1] as u64)
                }
            })
            .collect::<Vec<_>>()
    }
}

fn min_score(map: &Array2<bool>) -> Option<u64> {
    let mut lowest_score: HashMap<Ix2, u64> = HashMap::new();
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
            .into_iter()
            .for_each(|next| {
                let lowest = lowest_score.entry(next.position).or_insert(u64::MAX);
                if next.cost <= *lowest {
                    p_queue.push(next.clone());
                    // *lowest = next.cost;
                }
            })
    }

    None
}

pub fn day_18() {
    println!("--- Day 18 ---");

    let map: Array2<bool> = Array2::default((71, 71));

    {
        let mut map = map.clone();    

        INPUT.lines()
            .take(1024)
            .for_each(|l| {
                let mut split = l.split(',');
                let (x, y) = l.split(',').take(2).map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                map[(y, x)] = true;
            });

        let min_score = min_score(&map);
        println!("Minimum distance: {min_score:?}");
    }

    {
        let mut map = map.clone();

        let first_impassable_byte = INPUT.lines()
            .map(|l| {
                let mut split = l.split(',');
                let (x, y) = l.split(',').take(2).map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                map[(y, x)] = true;

                ((x, y), min_score(&map))
            })
            .skip_while(|(pos, score)| score.is_some())
            .next().unwrap().0;

        println!("First byte to block off path: {first_impassable_byte:?}");
    }
}