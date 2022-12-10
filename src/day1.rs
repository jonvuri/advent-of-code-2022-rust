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

#[cfg(test)]
mod tests {
    use crate::day1::generator;

    use super::part1;

    #[test]
    fn parses() {
        let input = "100\n200\n\n100\n100";
        assert_eq!(generator(&input), vec![vec![100, 200], vec![100, 100]]);
    }

    #[test]
    fn finds_max() {
        let input: Vec<Vec<u32>> = vec![vec![100, 200], vec![100, 100]];
        assert_eq!(part1(&input), 300);
    }
}
