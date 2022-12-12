use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<Vec<char>> {
    parser!(lines(alpha+))
        .parse(&utils::newline_end(&input))
        .unwrap()
}

// Will panic if result not between 1 and 52 inclusive
fn char_priority(ch: &char) -> u32 {
    match ch {
        'A'..='Z' => *ch as u32 - 'A' as u32 + 27,
        'a'..='z' => *ch as u32 - 'a' as u32 + 1,
        _ => unreachable!(),
    }
}

// Return char index into a char_counts array
fn char_count_index(ch: &char) -> usize {
    // No need to bounds check, char_priority already does
    usize::try_from(char_priority(ch)).unwrap() - 1
}

fn char_counts(chars: &[char]) -> [u32; 52] {
    let mut char_counts: [u32; 52] = [0; 52];

    for ch in chars {
        char_counts[char_count_index(ch)] += 1;
    }

    char_counts
}

pub fn part1(input: &[Vec<char>]) -> Result<u32, &'static str> {
    input
        .iter()
        .map(|line| {
            if !(line.len() > 0 && line.len() % 2 == 0) {
                return Err("Invalid line length");
            }

            let midpoint = line.len() / 2;

            let first = &line[..midpoint];
            let second = &line[midpoint..];

            let first_char_counts = char_counts(first);

            for ch in second {
                if first_char_counts[char_count_index(ch)] > 0 {
                    return Ok(char_priority(ch));
                }
            }

            Err("Invalid line: No item in both halves")
        })
        .sum()
}

pub fn part2(input: &[Vec<char>]) -> Result<u32, &'static str> {
    input
        .chunks(3)
        .map(|chunk| {
            if let [first, second, third] = &chunk[..] {
                let first_char_counts = char_counts(first);
                let second_char_counts = char_counts(second);

                for ch in third {
                    let count_index = char_count_index(ch);
                    if first_char_counts[count_index] > 0 && second_char_counts[count_index] > 0 {
                        return Ok(char_priority(ch));
                    }
                }

                Err("Invalid group: No common character found")
            } else {
                Err("Invalid input: Group of less than three lines")
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::generator;

    use super::part1;
    use super::part2;

    static EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn parses() {
        let chars: Vec<Vec<char>> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect(),
            "PmmdzqPrVvPwwTWBwg".chars().collect(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect(),
            "ttgJtRGJQctTZtZT".chars().collect(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect(),
        ];
        assert_eq!(generator(EXAMPLE_INPUT), chars);
    }

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), Ok(157));
    }

    #[test]
    fn part2_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part2(&input), Ok(70));
    }
}
