use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    parser!(sections(lines(u32)))
        .parse(&utils::newline_end(&input))
        .unwrap()
}

pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let elves = input.iter().map(|section| section.iter().sum());

    elves.max().unwrap()
}

pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    use std::collections::BinaryHeap;

    let elves = input.iter().map(|section| section.iter().sum());

    // BinaryHeap is a max-heap by default
    let mut heap = BinaryHeap::<u32>::from_iter(elves);

    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day1::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "100\n200\n\n100\n100\n\n250\n100";

    #[test]
    fn parses_example() {
        assert_eq!(
            generator(EXAMPLE_INPUT),
            vec![vec![100, 200], vec![100, 100], vec![250, 100]]
        );
    }

    #[test]
    fn part1_finds_max() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part1(&input), 350);
    }

    #[test]
    fn part2_sums_top_3() {
        let input = generator(EXAMPLE_INPUT);

        assert_eq!(part2(&input), 850);
    }
}
