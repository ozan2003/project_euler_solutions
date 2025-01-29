use project_euler::project_euler_solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::sync::LazyLock;

project_euler_solution!(022);

static ALPHABET_INDEX: LazyLock<HashMap<u8, usize>> =
    LazyLock::new(|| (b'A'..=b'Z').zip(1..).collect());

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
