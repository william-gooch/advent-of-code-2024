use std::collections::HashSet;

const INPUT: &str = include_str!("./input/day_6.txt");

#[derive(PartialEq, Clone)]
enum TileState {
    Empty,
    Visited,
    Wall,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '>' => Ok(Direction::Left),
            '<' => Ok(Direction::Right),
            _ => Err("Invalid direction")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct GuardPosition {
    pos: (isize, isize),
    dir: Direction,
}

impl GuardPosition {
    const fn next_position(&self) -> (isize, isize) {
        match self.dir {
            Direction::Up => (self.pos.0 - 1, self.pos.1),
            Direction::Down => (self.pos.0 + 1, self.pos.1),
            Direction::Left => (self.pos.0, self.pos.1 - 1),
            Direction::Right => (self.pos.0, self.pos.1 + 1),
        }
    }

    fn move_guard(&mut self, map: &Vec<Vec<TileState>>) -> bool {
        let (next_row, next_col) = self.next_position();

        if next_row < 0 || next_col < 0 || next_row >= map.len() as isize || next_col >= map.first().unwrap().len() as isize {
            true
        } else {
            match map[next_row as usize][next_col as usize] {
                TileState::Wall => {
                    self.dir.turn_right();
                },
                TileState::Empty | TileState::Visited => {
                    self.pos = (next_row, next_col);
                },
            }
            false
        }
    }
}

pub fn day_6() {
    println!("--- Day 6 ---");

    let mut guard_pos: Option<GuardPosition> = None;
    let map = INPUT.lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| -> Result<TileState, &str> {
                    match c {
                        '.' => Ok(TileState::Empty),
                        '#' => Ok(TileState::Wall),
                        _ => {
                            let dir = c.try_into()
                                .map_err(|_| "Invalid tile type")?;
                            guard_pos = Some(GuardPosition { pos: (row as isize, col as isize), dir });
                            Ok(TileState::Visited)
                        },
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let guard_pos = guard_pos.expect("No guard position found.");

    {
        let mut map = map.clone();
        let mut guard_pos = guard_pos.clone();
        let mut tiles_visited = 1;
        while !guard_pos.move_guard(&map) {
            let (row, col) = guard_pos.pos;
            if map[row as usize][col as usize] == TileState::Empty {
                tiles_visited += 1;
                map[row as usize][col as usize] = TileState::Visited;
            }
        }

        println!("Tiles visited: {tiles_visited}");
    }

    {
        let rows = map.len();
        let cols = map.first().unwrap().len();
        
        let possible_obstructions = (0..rows)
            .map(|row| {
                (0..cols)
                    .filter(|&col| {
                        if map[row][col] == TileState::Wall { return false; }

                        let mut positions: HashSet<GuardPosition> = HashSet::new();
                        let mut map = map.clone();
                        map[row][col] = TileState::Wall;
                        let map = map;
                        let mut guard_pos = guard_pos.clone();

                        positions.insert(guard_pos.clone());

                        while !guard_pos.move_guard(&map) {
                            if !positions.insert(guard_pos.clone()) {
                                println!("Obstruction at {row} {col}");
                                return true;
                            }
                        }

                        false
                    })
                    .count()
            })
            .sum::<usize>();

        println!("Possible obstructions: {possible_obstructions}");
    }
}