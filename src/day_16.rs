use std::{collections::{BinaryHeap, HashMap, HashSet}, ops::Add};

use ndarray::{Ix2, Array2};

const INPUT: &str = include_str!("./input/day_16.txt");
// const INPUT: &str = r"###############
// #.......#....E#
// #.#.###.#.###.#
// #.....#.#...#.#
// #.###.#####.#.#
// #.#.#.......#.#
// #.#.#####.###.#
// #...........#.#
// ###.#.#####.#.#
// #...#.....#.#.#
// #.#.#.###.#.#.#
// #.....#...#.#.#
// #.###.#.#.#.#.#
// #S..#.....#...#
// ###############";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn cw(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn ccw(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

impl Add<Direction> for Ix2 {
    type Output = Ix2;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Ix2(self[0] - 1, self[1]),
            Direction::Down => Ix2(self[0] + 1, self[1]),
            Direction::Left => Ix2(self[0], self[1] - 1),
            Direction::Right => Ix2(self[0], self[1] + 1),
        }
    }
}

struct Map {
    map: Array2<bool>,
    start_pos: Ix2,
    end_pos: Ix2,
}

impl Map {
    fn get(&self, pos: Ix2) -> bool {
        self.map[pos]
    }

    fn min_score(&self) -> Vec<State> {
        let mut lowest_score: HashMap<(Ix2, Direction), u64> = HashMap::new();
        let mut p_queue = BinaryHeap::new();
        p_queue.push(State { score: 0, position: self.start_pos, direction: Direction::Right, prev_positions: HashSet::new() });

        let mut paths: Vec<State> = Vec::new();
        while let Some(state) = p_queue.pop() {
            if state.position == self.end_pos {
                if paths.get(0).is_none_or(|p| state.score == p.score) {
                    paths.push(state.clone());
                } else {
                    return paths;
                }
            }

            let lowest = lowest_score.entry((state.position, state.direction)).or_insert(u64::MAX);
            if state.score > *lowest { continue; }

            state.possible_next_states(self)
                .into_iter()
                .for_each(|next| {
                    let lowest = lowest_score.entry((next.position, next.direction)).or_insert(u64::MAX);
                    if next.score <= *lowest {
                        p_queue.push(next.clone());
                        *lowest = next.score;
                    }
                })
        }

        paths
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let (map_flat, map_width, start_pos, end_pos) = value.lines()
            .enumerate()
            .fold((vec![], None, None, None), |(
                mut v,
                mut map_width,
                mut start_pos,
                mut end_pos,
            ), (row, l)| {
                map_width.get_or_insert(l.len());
                l.chars().enumerate().filter_map(|(col, c)| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    'S' => {
                        start_pos.replace(Ix2(row, col));
                        Some(false)
                    },
                    'E' => {
                        end_pos.replace(Ix2(row, col));
                        Some(false)
                    },
                    _ => None
                }).collect_into(&mut v);
                (v, map_width, start_pos, end_pos)
            });

        let map_width = map_width.expect("Map width not set");
        let map_height = map_flat.len() / map_width;

        let start_pos = start_pos.expect("Start position not set");
        let end_pos = end_pos.expect("End position not set");

        let map = Array2::from_shape_vec((map_height, map_width), map_flat).expect("Flat map was not of correct length");

        Self {
            map,
            start_pos,
            end_pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    score: u64,
    position: Ix2,
    direction: Direction,
    prev_positions: HashSet<Ix2>,
}

// flip ordering for a min-heap rather than a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn possible_next_states(&self, map: &Map) -> Vec<State> {
        let mut prev_positions = self.prev_positions.clone();
        prev_positions.insert(self.position);
        if map.get(self.position + self.direction) {
            vec![
                State { score: self.score + 1000, position: self.position, direction: self.direction.cw(), prev_positions: prev_positions.clone() },
                State { score: self.score + 1000, position: self.position, direction: self.direction.ccw(), prev_positions: prev_positions.clone() },
            ]
        } else {
            vec![
                State { score: self.score + 1000, position: self.position, direction: self.direction.cw(), prev_positions: prev_positions.clone() },
                State { score: self.score + 1000, position: self.position, direction: self.direction.ccw(), prev_positions: prev_positions.clone() },
                State { score: self.score + 1, position: self.position + self.direction, direction: self.direction, prev_positions: prev_positions.clone() },
            ]
        }
    }
}

pub fn day_16() {
    println!("--- Day 16 ---");

    let map: Map = INPUT.into();
    let min_paths = map.min_score();
    
    let min_score = min_paths.first().unwrap().score;
    println!("Minimum score: {min_score:?}");

    let mut tiles = min_paths.into_iter()
        .fold(HashSet::new(), |acc, el| {
            acc.union(&el.prev_positions).copied().collect()
        });
    tiles.insert(map.end_pos);
    println!("Number of tiles: {}", tiles.len());
}