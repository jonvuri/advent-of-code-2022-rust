use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    parser!(sections(lines(u32)))
        .parse(&utils::newline_end(&input))
        .unwrap()
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|group| group.iter().sum()).max().unwrap()
}

pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::<u32>::from_iter(input.iter().map(|group| group.iter().sum()));

    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day1::generator;

    use super::part1;
    use super::part2;

    #[test]
    fn parses() {
        let input = "100\n200\n\n100\n100";
        assert_eq!(generator(&input), vec![vec![100, 200], vec![100, 100]]);
    }

    #[test]
    fn part1_finds_max() {
        let input: Vec<Vec<u32>> = vec![vec![100, 200], vec![100, 100]];
        assert_eq!(part1(&input), 300);
    }

    #[test]
    fn part2_sums_top_3() {
        let input: Vec<Vec<u32>> = vec![vec![100, 200], vec![100, 100], vec![250, 100]];
        assert_eq!(part2(&input), 850);
    }
}
