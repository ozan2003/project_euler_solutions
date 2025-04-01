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

/// # Names Scores
///
/// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
/// containing over five-thousand first names, begin by sorting it into
/// alphabetical order. Then working out the alphabetical value for each name,
/// multiply this value by its alphabetical position in the list to obtain a
/// name score.
///
/// For example, when the list is sorted into alphabetical order, COLIN, which
/// is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN
/// would obtain a score of 938 × 53 = 49714.
///
/// What is the total of all the name scores in the file?
fn project_euler_022() -> usize
{
    let proj_dir = std::env::current_dir().unwrap();

    let file = File::open(proj_dir.join("data/022.txt")).expect("Couldn't find file.");

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
