use std::ops::Add;

use ndarray::{Array2, Ix2};

const INPUT: &str = include_str!("./input/day_15.txt");

type Position = Ix2;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    #[default]
    Empty,
    Box,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Ix2(self[0] - 1, self[1]),
            Direction::Down => Ix2(self[0] + 1, self[1]),
            Direction::Left => Ix2(self[0], self[1] - 1),
            Direction::Right => Ix2(self[0], self[1] + 1),
        }
    }
}

fn from_input(input: &str) -> (State, Vec<Direction>) {
    let (map_flat, map_width, robot) = input.lines()
        .take_while(|l| l.len() > 0)
        .enumerate()
        .fold((vec![], None, None), |(mut v, mut map_width, mut robot), (row, l)| {
            map_width.get_or_insert(l.len());
            l.chars().enumerate().filter_map(|(col, c)| match c {
                '.' => Some(Tile::Empty),
                '@' => {
                    robot.replace(Ix2(row, col));
                    Some(Tile::Empty)
                },
                'O' => Some(Tile::Box),
                '#' => Some(Tile::Wall),
                _ => None
            }).collect_into(&mut v);
            (v, map_width, robot)
        });

    let map_width = map_width.expect("Map width not set");
    let robot = robot.expect("Robot position not set");
    let map_height = map_flat.len() / map_width;

    let map = Array2::from_shape_vec((map_height, map_width), map_flat).expect("Flat map was not of correct length");


    let instructions = INPUT.lines()
        .skip_while(|l| l.len() > 0).skip(1)
        .flat_map(|l| {
            l.chars().filter_map(|c| {
                match c {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None
                }
            })
        })
        .collect::<Vec<_>>();

    (State { map, robot }, instructions)
}

#[derive(Debug, Clone)]
struct State {
    map: Array2<Tile>,
    robot: Position,
}

impl State {
    fn widen_tiles(&mut self) {
        let mut new_map: Array2<Tile> = Array2::default((self.map.shape()[0], self.map.shape()[1] * 2));

        self.map.indexed_iter()
            .for_each(|((row, col), tile)| {
                match tile {
                    Tile::Box => {
                        new_map[(row, col * 2)] = Tile::BoxLeft;
                        new_map[(row, col * 2 + 1)] = Tile::BoxRight;
                    },
                    t => {
                        new_map[(row, col * 2)] = *t;
                        new_map[(row, col * 2 + 1)] = *t;
                    },
                }
            });
        
        self.map = new_map;
        self.robot = Ix2(self.robot[0], self.robot[1] * 2);
    }

    fn move_direction(&mut self, direction: Direction) {
        if self.can_move_box(self.robot + direction, direction, true) {
            self.move_box(self.robot + direction, direction, true);
            self.robot = self.robot + direction;
        }
    }

    fn can_move_box(&mut self, position: Position, direction: Direction, check_other_half: bool) -> bool {
        match self.map[position] {
            Tile::Wall => false,
            Tile::Empty => true,
            Tile::Box => self.can_move_box(position + direction, direction, true),
            Tile::BoxLeft => {
                let move_in_direction = self.can_move_box(position + direction, direction, true);
                let move_other_half = if direction != Direction::Left && direction != Direction::Right && check_other_half {
                    self.can_move_box(position + Direction::Right, direction, false)
                } else { true };

                move_in_direction && move_other_half
            },
            Tile::BoxRight => {
                let move_in_direction = self.can_move_box(position + direction, direction, true);
                let move_other_half = if direction != Direction::Left && direction != Direction::Right && check_other_half {
                    self.can_move_box(position + Direction::Left, direction, false)
                } else { true };

                move_in_direction && move_other_half
            },
        }
    }

    fn move_box(&mut self, position: Position, direction: Direction, move_other_half: bool) {
        match self.map[position] {
            Tile::Wall => panic!("Shouldn't move wall!"),
            Tile::Empty => (),
            Tile::Box => {
                self.move_box(position + direction, direction, true);
                self.map[position + direction] = Tile::Box;
            },
            Tile::BoxLeft => {
                self.move_box(position + direction, direction, true);
                if direction != Direction::Left && direction != Direction::Right && move_other_half {
                    self.move_box(position + Direction::Right, direction, false);
                }
                self.map[position + direction] = Tile::BoxLeft;
            },
            Tile::BoxRight => {
                self.move_box(position + direction, direction, true);
                if direction != Direction::Left && direction != Direction::Right && move_other_half {
                    self.move_box(position + Direction::Left, direction, false);
                }
                self.map[position + direction] = Tile::BoxRight;
            },
        }
        self.map[position] = Tile::Empty;
    }

    fn sum_gps_coords(&self) -> u64 {
        self.map.indexed_iter()
            .filter_map(|((row, col), tile)| {
                if *tile != Tile::Box && *tile != Tile::BoxLeft { None }
                else {
                    Some((row as u64) * 100 + (col as u64))
                }
            })
            .sum()
    }

    fn print_map(&self) {
        self.map.rows().into_iter()
            .enumerate()
            .for_each(|(row, r)| {
                r.iter().enumerate().for_each(|(col, t)| {
                    if self.robot == Ix2(row, col) { print!("@") }
                    else {
                        match t {
                            Tile::Empty => print!("."),
                            Tile::Box => print!("O"),
                            Tile::Wall => print!("#"),
                            Tile::BoxLeft => print!("["),
                            Tile::BoxRight => print!("]"),
                        }
                    }
                });
                println!("");
            });
    }
}

pub fn day_15() {
    println!("--- Day 15 ---");

    let (state, instructions) = from_input(INPUT);

    {
        let mut state = state.clone();

        instructions.iter()
            .for_each(|dir| {
                state.move_direction(*dir);
            });

        let total = state.sum_gps_coords();
        
        println!("Sum GPS coords: {total:?}");
    }

    {
        let mut state = state.clone();
        state.widen_tiles();

        instructions.iter()
            .for_each(|dir| {
                state.move_direction(*dir);
            });

        let total = state.sum_gps_coords();
        
        println!("Sum GPS coords (widened): {total:?}");
    }
}