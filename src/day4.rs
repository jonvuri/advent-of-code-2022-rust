use aoc_parse::{parser, prelude::*};

use crate::utils;

type Assignments = (u32, u32, u32, u32);

pub fn generator(input: &str) -> Vec<Assignments> {
    parser!(lines(
       u32 "-" u32 "," u32 "-" u32
    ))
    .parse(&utils::newline_end(input))
    .unwrap()
}

pub fn part1(input: &[Assignments]) -> u32 {
    input
        .iter()
        .filter(|(start0, end0, start1, end1)| {
            (start0 >= start1 && end0 <= end1) || (start0 <= start1 && end0 >= end1)
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, Assignments};

    static EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn parses() {
        let chars: Vec<Assignments> = vec![
            (2, 4, 6, 8),
            (2, 3, 4, 5),
            (5, 7, 7, 9),
            (2, 8, 3, 7),
            (6, 6, 4, 6),
            (2, 6, 4, 8),
        ];
        assert_eq!(generator(EXAMPLE_INPUT), chars);
    }

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), 2);
    }
}
