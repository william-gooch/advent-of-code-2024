use std::rc::Rc;

use itertools::Itertools;
use fxhash::FxHashMap;
use trie_rs::Trie;

const INPUT: &str = include_str!("./input/day_19.txt");
const TEST_INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for Color {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Color::White),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            _ => Err(())
        }
    }
}

impl From<Color> for char {
    fn from(value: Color) -> Self {
        match value {
            Color::White => 'w',
            Color::Blue => 'u',
            Color::Black => 'b',
            Color::Red => 'r',
            Color::Green => 'g',
        }
    }
}

impl FromIterator<Color> for String {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        iter.into_iter()
            .map(|col| char::from(col))
            .collect()
    }
}

impl<'a> FromIterator<&'a Color> for String {
    fn from_iter<T: IntoIterator<Item = &'a Color>>(iter: T) -> Self {
        iter.into_iter()
            .map(|col| char::from(*col))
            .collect()
    }
}

fn from_input(input: &str) -> (Trie<Color>, Vec<Vec<Color>>) {
    let mut lines = input.lines();

    let patterns: Trie<Color> = lines
        .take_while_ref(|l| l.len() > 0)
        .flat_map(|l| {
            l.split(", ")
                .map(|pat| pat
                        .chars()
                        .filter_map(|c| Color::try_from(c).ok())
                        .collect::<Vec<_>>())
        })
        .collect();

    let designs: Vec<Vec<Color>> = lines.skip(1)
        .map(|design| {
            design
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect()
        })
        .collect::<Vec<_>>();

    (patterns, designs)
}

fn try_make_design(patterns: &Trie<Color>, design: &[Color], cache: &mut FxHashMap<Rc<[Color]>, usize>) -> usize {
    if design.len() == 0 { return 1 }

    if let Some(&num_patterns) = cache.get(design) {
        num_patterns
    } else {
        let mut prefixes: Vec<Vec<Color>> = patterns.common_prefix_search(design).collect();
        prefixes.sort_by(|a, b| b.len().cmp(&a.len()));

        let num_patterns = prefixes.iter()
            .map(|prefix| {
                let num_patterns = try_make_design(patterns, &design[prefix.len()..], cache);

                num_patterns
            })
            .sum();

        cache.insert(Rc::from(design), num_patterns);

        num_patterns
    }
}

pub fn day_19() {
    println!("--- Day 19 ---");

    let (patterns, designs) = from_input(INPUT);

    let mut cache: FxHashMap<Rc<[Color]>, usize> = Default::default();
    let (possibles_a, possibles_b) = designs.iter()
        .map(|design| {
            try_make_design(&patterns, design, &mut cache)
        })
        .tee();

    let num_possible: usize = possibles_a.filter(|n| *n > 0).count();
    let ways_possible: usize = possibles_b.sum();

    println!("Number of possible patterns: {num_possible}");
    println!("Number of ways to combine: {ways_possible}");
}