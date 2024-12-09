use std::{fmt::{Debug, Display}, iter::repeat};

use itertools::Itertools;

const INPUT: &str = include_str!("./input/day_9.txt");
// const INPUT: &str = "48454";

#[derive(Clone)]
struct File {
    file_id: u16,
    file_len: usize,
    file_gap: usize,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_id)
    }
}

fn print_fs(fs: &Vec<File>) {
    fs.iter()
        .flat_map(|file| {
            let file_chunks = repeat(Some(file.file_id)).take(file.file_len);
            let gap_chunks = repeat(None).take(file.file_gap);

            file_chunks.chain(gap_chunks)
        })
        .for_each(|block| {
            print!("{}", block.map(|b| b.to_string()).unwrap_or(".".to_owned()))
        });

    println!("");
}

pub fn day_9() {
    println!("--- Day 9 ---");

    let files = INPUT.chars()
        .filter(|c| c.is_ascii_digit())
        .chunks(2)
        .into_iter()
        .enumerate()
        .filter_map(|(file_id, mut chunk)| {
            let file_len = chunk.next()?.to_digit(10)? as usize;
            let file_gap = chunk.next().and_then(|c| Some(c.to_digit(10)? as usize)).unwrap_or(0);

            Some(File { file_id: file_id as u16, file_len, file_gap })
        })
        .collect::<Vec<_>>();

    let mut blocks = files.iter()
        .flat_map(|file| {
            let file_chunks = repeat(Some(file.file_id)).take(file.file_len);
            let gap_chunks = repeat(None).take(file.file_gap);

            file_chunks.chain(gap_chunks)
        })
        .collect::<Vec<_>>();

    (0..blocks.len()).rev()
        .map_while(|i| {
            let first_gap_idx = blocks.iter()
                .enumerate()
                .take(i - 1)
                .skip_while(|&(_, b)| b.is_some()).next()?.0;

            blocks[first_gap_idx] = blocks[i];
            blocks[i] = None;

            Some(())
        })
        .for_each(|_| ());

    let checksum: u64 = blocks.iter()
        .enumerate()
        .filter_map(|(i, &b)| {
            Some((i as u64) * (b? as u64))
        })
        .sum();

    println!("Filesystem checksum: {checksum}");

    let mut files = files.clone();
    print_fs(&files);
    let file_ids = files.iter().map(|f| f.file_id).collect::<Vec<_>>();
    file_ids.into_iter()
        .skip(1)
        .rev()
        .for_each(|file_id| {
            let file_to_move = files.iter().position(|f| f.file_id == file_id).unwrap();
            let required_len = files[file_to_move].file_len;
            let first_sufficient_gap = files.iter().take(file_to_move).position(|f| required_len <= f.file_gap);

            if let Some(gap) = first_sufficient_gap {
                let old_file = files[file_to_move].clone();
                let mut moved_file = File {
                    file_id: old_file.file_id,
                    file_len: old_file.file_len,
                    file_gap: files[gap].file_gap - old_file.file_len,
                };

                files[gap].file_gap = 0; // set gap before inserted file to zero
                if gap == file_to_move - 1 { // edge case: if the available gap is immediately before
                    moved_file.file_gap += old_file.file_len + old_file.file_gap;
                } else {
                    files[file_to_move - 1].file_gap += old_file.file_len + old_file.file_gap; // fill the remaining space with gap
                }

                files.remove(file_to_move);
                files.insert(gap + 1, moved_file);
            }

            // print!("{file_id}: ");
            // print_fs(&files);
        });

    let mut blocks = files.iter()
        .flat_map(|file| {
            let file_chunks = repeat(Some(file.file_id)).take(file.file_len);
            let gap_chunks = repeat(None).take(file.file_gap);

            file_chunks.chain(gap_chunks)
        })
        .collect::<Vec<_>>();

    let checksum: u64 = blocks.iter()
        .enumerate()
        .filter_map(|(i, &b)| {
            Some((i as u64) * (b? as u64))
        })
        .sum();
    println!("Filesystem checksum: {checksum}");
}
