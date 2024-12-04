const INPUT: &str = include_str!("./input/day_4.txt");

pub fn day_4() {
    println!("--- Day 4 ---");

    let dirs_to_check = |(row, col): (usize, usize), (max_row, max_col): (usize, usize)| {
        let row = row as isize;
        let col = col as isize;
        (-1..=1)
            .flat_map(|i: isize| (-1..=1).filter_map(move |j: isize| {
                if i == 0 && j == 0 { None }
                else if row + (i*3) < 0 || col + (j*3) < 0 { None }
                else if row + (i*3) >= (max_row as isize) || col + (j*3) >= (max_col as isize) { None }
                else {
                    Some(
                        (0..=3).map(|d| {
                            ((row + i * d) as usize, (col + j * d) as usize)
                        })
                        .collect::<Vec<_>>()
                    )
                }
            }))
            .collect::<Vec<_>>()
    };

    let line_chars = INPUT.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_row = line_chars.len();
    let max_col = line_chars[0].len();

    let matches = line_chars.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, char)| {
                    (*char == 'X').then(|| {
                        dirs_to_check((row, col), (max_row, max_col))
                            .iter()
                            .filter(|d| {
                                d.iter()
                                    .zip(['X', 'M', 'A', 'S'])
                                    .all(|((i, j), c)| {
                                        line_chars[*i][*j] == c
                                    })
                            })
                            .count()
                    })
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("total matches: {}", matches);

    let crosses_to_check = |(row, col): (usize, usize), (max_row, max_col): (usize, usize)| {
        let row = row as isize;
        let col = col as isize;
        let max_row = max_row as isize;
        let max_col = max_col as isize;
        [
            (-1, -1),
            ( 1, -1),
            (-1,  1),
            ( 1,  1),
        ]
            .into_iter()
            .filter_map(|(i, j)| {
                if row + i >= max_row
                || row - i >= max_row
                || row + i < 0
                || row - i < 0
                || col + j >= max_col
                || col - j >= max_col
                || col + j < 0
                || col - j < 0 {
                    None
                } else {
                    Some(vec![((row + i) as usize, (col + j) as usize), (row as usize, col as usize), ((row - i) as usize, (col - j) as usize)])
                }
            })
            .collect::<Vec<_>>()
    };

    let xmas_matches = line_chars.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(col, char)| {
                    (**char == 'A').then(|| {
                        crosses_to_check((row, *col), (max_row, max_col))
                            .into_iter()
                            .filter(|x| {
                                x.iter()
                                    .zip(['M', 'A', 'S'])
                                    .all(|((i, j), c)| line_chars[*i][*j] == c)
                            })
                            .count() >= 2
                    }).is_some_and(|b| b)
                })
                .count()
        })
        .sum::<usize>();

    println!("xmas matches: {xmas_matches}");
}
