use phf::{Map, phf_map};
use project_euler::project_euler_solution;
use std::fs::File;
use std::io::{BufReader, prelude::*};

project_euler_solution!(022);

const ALPHABET_INDEX: Map<u8, usize> = phf_map! {
    b'A' => 1, b'B' => 2, b'C' => 3, b'D' => 4, b'E' => 5,
    b'F' => 6, b'G' => 7, b'H' => 8, b'I' => 9, b'J' => 10,
    b'K' => 11, b'L' => 12, b'M' => 13, b'N' => 14, b'O' => 15,
    b'P' => 16, b'Q' => 17, b'R' => 18, b'S' => 19, b'T' => 20,
    b'U' => 21, b'V' => 22, b'W' => 23, b'X' => 24, b'Y' => 25,
    b'Z' => 26
};

fn name_score(name: &[u8]) -> usize
{
    name.iter()
        .map(|ch| ALPHABET_INDEX[ch])
        .sum()
}

fn project_euler_022() -> usize
{
    let proj_dir = std::env::current_dir().unwrap();

    let file =
        File::open(format!("{}/data/022.txt", proj_dir.display())).expect("Couldn't find file.");

    let buf = BufReader::new(file);

    let mut names: Vec<Vec<u8>> = buf
        .split(b',')
        .map(|name| {
            name.unwrap()
                .iter()
                // trim the quotes
                .skip(1)
                .take_while(|&&ch| ch != b'"')
                .copied()
                .collect()
        })
        .collect();

    names.sort_unstable();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| (i + 1) * name_score(name))
        .sum()
}
