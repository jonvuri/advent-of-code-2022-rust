use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

use crate::utils;

pub enum Line {
    ChangeDirectoryCommand { directory: String },
    ListCommand,
    DirectoryListItem { directory: String },
    FileListItem { size: u32 },
}

pub fn generator(input: &str) -> Vec<Line> {
    parser!(lines({
        "$ cd " arg:string(any_char+) => Line::ChangeDirectoryCommand { directory: arg },
        "$ ls" => Line::ListCommand,
        "dir " arg:string(any_char+) => Line::DirectoryListItem { directory: arg },
        size:u32 " " any_char+ => Line::FileListItem { size },
    }))
    .parse(&utils::newline_end(input))
    .unwrap()
}

fn get_directory_sizes(input: &Vec<Line>) -> Vec<u32> {
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    let mut directory_relationships: Vec<(String, String)> = Vec::new();

    let mut current_directory = "/".to_owned();

    // First pass: Initialize directory_sizes with only immediate file children sizes
    for line in input {
        match line {
            Line::ChangeDirectoryCommand { directory } => match directory.as_str() {
                ".." => {
                    let mut parts = current_directory.split("/").collect::<Vec<&str>>();
                    if parts.len() > 1 {
                        parts.pop();
                    }
                    current_directory = parts.join("/");
                }
                "/" => {
                    current_directory = "/".to_owned();
                }
                _ => {
                    current_directory = format!("{}/{}", current_directory, directory);
                }
            },
            Line::ListCommand => {}
            Line::DirectoryListItem { directory } => {
                directory_relationships.push((current_directory.clone(), directory.clone()));
            }
            Line::FileListItem { size } => {
                directory_sizes
                    .entry(current_directory.clone())
                    .and_modify(|d| *d += *size)
                    .or_insert(*size);
            }
        }
    }

    // Second pass: Add all children sizes to parent directories
    // We assume that directory relationships are listed in order of depth
    // from the command list, so going in reverse order ensures all children
    // of a parent come before the parent itself.
    for (parent, child) in directory_relationships.iter().rev() {
        let child_size = directory_sizes
            .get(&format!("{}/{}", parent, child))
            .unwrap()
            .clone();
        directory_sizes
            .entry(parent.clone())
            .and_modify(|d| *d += child_size)
            .or_insert(child_size);
    }

    let mut sizes: Vec<u32> = directory_sizes.into_values().collect();
    sizes.sort();

    sizes
}

pub fn part1(input: &Vec<Line>) -> Result<u32, &'static str> {
    let sizes = get_directory_sizes(input);

    let result = sizes.into_iter().filter(|s| *s < 100000).sum::<u32>();

    Ok(result)
}

pub fn part2(input: &Vec<Line>) -> Result<u32, &'static str> {
    let sizes = get_directory_sizes(input);

    if sizes.len() == 0 {
        return Err("No directories");
    }

    let total_size = sizes.last().unwrap();

    if *total_size > 70000000 {
        return Err("Size out of bounds");
    }

    let unused_size = 70000000 - *total_size;

    if unused_size > 30000000 {
        return Err("Already have enough space");
    }

    let needed_size = 30000000 - unused_size;

    let result = sizes
        .into_iter()
        .filter(|s| *s > needed_size)
        .min()
        .unwrap();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), Ok(95437));
    }

    #[test]
    fn part2_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part2(&input), Ok(24933642));
    }
}
