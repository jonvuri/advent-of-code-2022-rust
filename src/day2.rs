use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<(char, char)> {
    parser!(lines(alpha " " alpha))
        .parse(&utils::newline_end(&input))
        .unwrap()
}

pub fn part1(input: &[(char, char)]) -> Result<u32, &'static str> {
    input
        .iter()
        .map(|line| match line {
            ('A', 'X') => Ok(1 + 3),
            ('A', 'Y') => Ok(2 + 6),
            ('A', 'Z') => Ok(3),

            ('B', 'X') => Ok(1),
            ('B', 'Y') => Ok(2 + 3),
            ('B', 'Z') => Ok(3 + 6),

            ('C', 'X') => Ok(1 + 6),
            ('C', 'Y') => Ok(2),
            ('C', 'Z') => Ok(3 + 3),

            _ => Err("Invalid input"),
        })
        .sum()
}

pub fn part2(input: &[(char, char)]) -> Result<u32, &'static str> {
    input
        .iter()
        .map(|line| match line {
            ('A', 'X') => Ok(3),
            ('A', 'Y') => Ok(1 + 3),
            ('A', 'Z') => Ok(2 + 6),

            ('B', 'X') => Ok(1),
            ('B', 'Y') => Ok(2 + 3),
            ('B', 'Z') => Ok(3 + 6),

            ('C', 'X') => Ok(2),
            ('C', 'Y') => Ok(3 + 3),
            ('C', 'Z') => Ok(1 + 6),

            _ => Err("Invalid input"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn parses() {
        assert_eq!(
            generator(EXAMPLE_INPUT),
            vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]
        );
    }

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), Ok(15));
    }

    #[test]
    fn part2_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part2(&input), Ok(12));
    }
}
