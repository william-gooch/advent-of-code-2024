use fxhash::FxHashMap as HashMap;
use std::{collections::BinaryHeap, fmt::Display, ops::Add, usize};
use ndarray::{Array2, Axis, Ix2};

const INPUT: &str = include_str!("./input/day_20.txt");
// const INPUT: &str = r"###############
// #...#...#.....#
// #.#.#.#.#.###.#
// #S#...#.#.#...#
// #######.#.#.###
// #######.#.#...#
// #######.#.###.#
// ###..E#...#...#
// ###.#######.###
// #...###...#...#
// #.#####.#.###.#
// #.#...#.#.#...#
// #.#.#.#.#.#.###
// #...#...#...###
// ###############";

fn manhattan(a: Ix2, b: Ix2) -> usize {
    a[0].abs_diff(b[0]) + a[1].abs_diff(b[1])
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

    fn dist_to_end(&self, pos: Ix2) -> usize {
        manhattan(pos, self.end_pos)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.rows().into_iter()
            .enumerate()
            .try_for_each(|(row, r)| {
                r.iter().enumerate().try_for_each(|(col, t)| {
                    if self.start_pos == Ix2(row, col) { write!(f, "S") }
                    else if self.end_pos == Ix2(row, col) { write!(f, "E") }
                    else {
                        match self.get(Ix2(row, col)) {
                            false => write!(f, "."),
                            true => write!(f, "#"),
                        }
                    }
                })?;
                writeln!(f, "")
            });

        Ok(())
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct DijkstraState {
    position: Ix2,
    cost: usize,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost).cmp(&(self.cost))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRS: [[isize; 2]; 4] = [
    [-1,  0],
    [ 1,  0],
    [ 0, -1],
    [ 0,  1],
];
fn dijkstra(map: &Map) -> (Array2<usize>, Vec<Ix2>) {
    let mut dists = Array2::from_shape_simple_fn(map.map.raw_dim(), || usize::MAX);
    let mut queue: BinaryHeap<DijkstraState> = BinaryHeap::new();
    queue.push(DijkstraState { position: map.end_pos, cost: 0 });

    let mut path: Vec<Ix2> = Vec::new();
    while let Some(state) = queue.pop() {
        if state.cost < dists[state.position] {
            dists[state.position] = state.cost;
            path.push(state.position);
            
            for dir in DIRS {
                let pos = Ix2(
                    state.position[0].wrapping_add_signed(dir[0]),
                    state.position[1].wrapping_add_signed(dir[1]),
                );

                if let Some(false) = map.map.get(pos) {
                    queue.push(DijkstraState { position: pos, cost: state.cost + 1 });
                }
            }
        }
    }

    path.windows(2)
        .for_each(|pos| {
            // make sure there's only a single path through
            assert!(dists[pos[1]] - dists[pos[0]] == 1);
        });

    (dists, path)
}

fn cheat_savings<'a>(map: &'a Map, dijkstra_map: &'a Array2<usize>, dijkstra_path: &'a Vec<Ix2>, cheat_distance: usize) -> impl Iterator<Item = (Ix2, Ix2, usize)> + use<'a> {
    (0..dijkstra_path.len()).rev()
        .flat_map(|range| {
            (0..(dijkstra_path.len() - range))
                .map(move |start| (start, start + range))
        })
        .map(|(start, end)| {
            let start_pos = dijkstra_path[start];
            let end_pos = dijkstra_path[end];

            (start_pos, end_pos)
        })
        .filter_map(move |(start_pos, end_pos)| {
            let manhattan_dist = manhattan(start_pos, end_pos);
            (manhattan_dist <= cheat_distance)
                .then(|| {
                    dijkstra_map[end_pos].checked_sub(dijkstra_map[start_pos] + manhattan_dist)
                })
                .flatten()
                .and_then(|saving| {
                    (saving > 0).then_some((start_pos, end_pos, saving))
                })
        })
}

pub fn day_20() {
    println!("--- Day 20 ---");

    let map: Map = INPUT.into();
    let (dijkstra_map, path) = dijkstra(&map);
    println!("Cost without cheats: {}", dijkstra_map[map.start_pos]);

    let num_cheats = cheat_savings(&map, &dijkstra_map, &path, 2)
        // .inspect(|s| println!("{s:?}"))
        .take_while(|(_, _, saving)| *saving >= 100)
        .count();
    println!("Number of cheats (cheat length = 2): {num_cheats}");

    let num_cheats = cheat_savings(&map, &dijkstra_map, &path, 20)
        // .inspect(|s| println!("{s:?}"))
        .take_while(|(_, _, saving)| *saving >= 80)
        .filter(|(_, _, saving)| *saving >= 100)
        .count();
    println!("Number of cheats (cheat length = 20): {num_cheats}");
}